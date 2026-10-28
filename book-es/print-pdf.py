#!/usr/bin/env python3
"""Prints the book to one PDF, with a contents list, page numbers, an index
and bookmarks.

mdBook's print.html has every chapter on one page, and Chrome prints it the
way a browser's Print button does. A printed page has no links to follow,
so this script adds what a printed book needs to find things in it:

- a title page, then the contents: every chapter of SUMMARY.md with the
  page it starts on;
- the page number at the bottom of every page (print.css);
- an index at the back: the words of the music and Rust glossaries and of
  index-words.txt, with the pages that explain them;
- bookmarks, the outline a PDF reader shows beside the pages: parts,
  chapters and their sections.

Chrome can't tell the HTML on which page something will land, so the page
is printed twice: the first time with blanks for the numbers, to find out
where every chapter and word landed, the second time with the numbers
filled in. The numbers take as much room as the blanks did, so nothing
moves; the script checks that, and prints once more if something did.

Run it in book/, after a build in the light theme into pdf/ (see
.github/workflows/book.yml):

    python3 print-pdf.py pdf book/ferrous-lineup-book.pdf

It needs pypdf (pip install pypdf) and Chrome; CHROME names another
Chrome or Chromium program than google-chrome.
"""

import html
import logging
import os
import re
import subprocess
import sys
import tempfile
import tomllib
import unicodedata
import urllib.parse
from dataclasses import dataclass, field
from pathlib import Path

from pypdf import PdfReader, PdfWriter
from pypdf.generic import ArrayObject, DictionaryObject, Fit, NameObject, TextStringObject

# mdBook puts this between chapters in print.html, so each one starts on a
# new page.
CHAPTER_BREAK = '<div style="break-before: page; page-break-before: always;"></div>'
HEADING = re.compile(
    r'<h([1-6]) id="([^"]+)"><a class="header" href="#[^"]+">(.*?)</a></h\1>', re.S
)
STRONG = re.compile(r"<strong>(.*?)</strong>", re.S)
GLOSSARY_ROW = re.compile(r"<tr><td>(.*?)</td><td>(.*?)</td><td>(.*?)</td></tr>", re.S)
# Wide enough for any page number of the book, so the real numbers fit
# where the blanks were.
BLANK = "0000"

STYLE = """
<style>
@page :first { @bottom-center { content: none; } }
.pdf-title-page { text-align: center; padding-top: 70mm; }
.pdf-title-page .title { font-size: 2.6em; font-weight: bold; margin: 0; }
.pdf-title-page .subtitle { font-size: 1.6em; margin: 0.4em 0 3em; }
/* In the blue of a link, so it shows that a click goes there. */
.content .pdf-contents a:link,
.content .pdf-contents a:visited { text-decoration: none; }
.pdf-contents .part { font-weight: bold; margin: 1.1em 0 0.3em; break-after: avoid; }
.pdf-contents .entry { display: flex; align-items: baseline; break-inside: avoid; margin: 0.1em 0; }
.pdf-contents .entry .dots {
    flex: 1; min-width: 1.5em; margin: 0 0.4em;
    border-bottom: 1px dotted currentColor; opacity: 0.5;
}
.pdf-contents .entry .page { font-variant-numeric: tabular-nums; }
.pdf-contents .depth-0 { font-weight: 600; margin-top: 0.5em; }
.pdf-contents .depth-1 { padding-left: 1.5em; }
.pdf-contents .depth-2 { padding-left: 3em; }
.pdf-contents .index { margin-top: 1.1em; }
.pdf-index { columns: 2; column-gap: 2em; font-size: 0.9em; }
.pdf-index .letter {
    font-weight: bold; font-size: 1.3em; margin: 0.8em 0 0.2em; break-after: avoid;
}
.pdf-index .letter:first-child { margin-top: 0; }
.pdf-index .item { margin: 0 0 0.2em 1.2em; text-indent: -1.2em; break-inside: avoid; }
</style>
"""


@dataclass
class Heading:
    level: int
    id: str
    title: str  # plain text


@dataclass
class Chapter:
    title: str  # as in SUMMARY.md, the sidebar's name for it
    path: str  # its file, as SUMMARY.md links it
    depth: int  # 0 for a top-level chapter, 1 for one under it, ...
    part: str | None  # the "# Part ..." heading it is under
    headings: list[Heading] = field(default_factory=list)

    @property
    def id(self) -> str:
        return self.headings[0].id


@dataclass
class Entry:
    """A word in the index."""

    label: str  # HTML, as the glossary writes it
    sort_key: str
    # (id, explained there): the places it points to.
    places: list[tuple[str, bool]] = field(default_factory=list)
    see: str | None = None  # another entry's label, for "see ..."
    glossary: bool = False  # a glossary word, looked for in bold
    # Chapters where the word in bold means something else.
    not_in: set[str] = field(default_factory=set)


def plain(fragment: str) -> str:
    """The text of an HTML fragment, without tags."""
    return html.unescape(re.sub(r"<[^>]+>", "", fragment)).strip()


def normal(text: str) -> str:
    """A word as compared with others: lower case, straight quotes, single
    spaces, no punctuation at the end."""
    text = text.lower().replace("’", "'").replace("‘", "'")
    return re.sub(r"\s+", " ", text).strip().rstrip(".:,;…").strip()


def forms(word: str) -> set[str]:
    """A word and its plural, as the book writes them: "inversion" also
    matches "inversions"."""
    word = normal(word)
    found = {word, word + "s", word + "es"}
    if word.endswith("y"):
        found.add(word[:-1] + "ies")
    # Spanish plurals that change the word: "inversión", "inversiones".
    if word.endswith("ión"):
        found.add(word[:-3] + "iones")
    return found


def read_summary(path: Path) -> list[Chapter]:
    chapters = []
    part = None
    for line in path.read_text(encoding="utf-8").splitlines():
        if m := re.match(r"^( *)(?:- )?\[(.+)\]\((.+)\)\s*$", line):
            chapters.append(Chapter(m[2], m[3], len(m[1]) // 2, part))
        elif line.startswith("# ") and line != "# Índice":
            part = line[2:].strip()
    return chapters


def sort_key(label: str) -> str:
    # Accents don't move a word: "Índice" sorts with the I's.
    text = unicodedata.normalize("NFD", normal(plain(label)))
    text = "".join(c for c in text if not unicodedata.combining(c))
    return re.sub(r"^[^0-9a-z]+", "", text)


def is_code(fragment: str) -> bool:
    """Whether a word is all code, like `windows`, which is not the
    Windows of "Linux, macOS and Windows"."""
    return re.fullmatch(r"\s*<code>[^<]*</code>\s*", fragment) is not None


def build_index(
    chunks: list[str], chapters: list[Chapter], words: list[str]
) -> tuple[list[str], list[Entry]]:
    """Finds the places for the index, and gives every bold glossary word
    in the chapters an id so the index can point at it. Returns the
    chapters with those ids added, and the entries."""
    entries: dict[str, Entry] = {}

    def entry(label: str) -> Entry:
        key = normal(plain(label))
        if key not in entries:
            entries[key] = Entry(label, sort_key(label))
        return entries[key]

    # Each glossary row: the word, what it means, and the chapter that
    # explains it, linked.
    glossaries = ("glosario-de-música", "glosario-de-rust")
    for chunk, chapter in zip(chunks, chapters, strict=True):
        if chapter.id not in glossaries:
            continue
        for word, meaning, where in GLOSSARY_ROW.findall(chunk):
            label = STRONG.search(word)
            target = re.search(r'href="#([^"]+)"', where)
            if not label or not target:
                continue
            explained = entry(label[1])
            explained.glossary = True
            explained.places.append((html.unescape(target[1]), True))
            # "Also called **modal mixture**": another name, pointing at
            # this one.
            for other in STRONG.findall(meaning):
                if normal(plain(other)) != normal(plain(label[1])):
                    entry(other).see = label[1]

    # index-words.txt: "Word: chapter.md#section, ...", "Word: see Other"
    # and "Word: not chapter.md".
    by_path = {chapter.path: chapter for chapter in chapters}
    for line in words:
        word, sep, where = line.partition(": ")
        if not sep:
            sys.exit(f"index-words.txt: no places for {line!r}")
        e = entry(html.escape(word))
        if where.startswith("véase "):
            e.see = html.escape(where[6:])
            continue
        for place in where.split(", "):
            exclude = place.startswith("no ")
            path, _, section = place.removeprefix("no ").partition("#")
            chapter = by_path.get(path)
            if chapter is None:
                sys.exit(f"index-words.txt: {word}: no chapter {path}")
            if exclude:
                e.not_in.add(chapter.id)
                continue
            id = chapter.id
            if section:
                # mdBook numbers a heading id that is already taken
                # somewhere in the book: "exercises-3".
                ids = [
                    h.id
                    for h in chapter.headings
                    if re.fullmatch(re.escape(section) + r"(-\d+)?", h.id)
                ]
                if not ids:
                    sys.exit(f"index-words.txt: {word}: no section {place}")
                id = ids[0]
            e.places.append((id, False))

    # Every glossary word, where a chapter writes it in bold: that is how
    # the book introduces a word. The glossaries themselves are left out:
    # their bold words are the entries.
    by_form: dict[tuple[str, bool], Entry] = {}
    for key, e in entries.items():
        if e.glossary:
            for f in forms(key):
                by_form.setdefault((f, is_code(e.label)), e)
    marked = []
    counter = 0
    for chunk, chapter in zip(chunks, chapters, strict=True):
        if chapter.id in glossaries:
            marked.append(chunk)
            continue

        def mark(m: re.Match, here: str = chapter.id) -> str:
            nonlocal counter
            e = by_form.get((normal(plain(m[1])), is_code(m[1])))
            if e is None or here in e.not_in:
                return m[0]
            counter += 1
            e.places.append((f"pdf-ix-{counter}", False))
            return f'<strong id="pdf-ix-{counter}">{m[1]}</strong>'

        marked.append(STRONG.sub(mark, chunk))

    for e in entries.values():
        if not e.places and not e.see:
            sys.exit(f"index: no page for {plain(e.label)!r}")
    return marked, sorted(entries.values(), key=lambda e: (e.sort_key, plain(e.label)))


def page_text(page: int | None) -> str:
    return BLANK if page is None else str(page + 1)


def heading(id: str, title: str) -> str:
    """A chapter heading as mdBook writes one: it links to itself, which
    is also what makes Chrome keep its place in the PDF for bookmarks."""
    return f'<h1 id="{id}"><a class="header" href="#{id}">{title}</a></h1>'


def contents_html(chapters: list[Chapter], pages: dict[str, int]) -> str:
    lines = [heading("pdf-contents", "Contenido"), '<nav class="pdf-contents">']
    part = None
    for chapter in chapters:
        if chapter.part != part:
            part = chapter.part
            lines.append(f'<p class="part">{html.escape(part)}</p>')
        lines.append(
            f'<a class="entry depth-{chapter.depth}" href="#{chapter.id}">'
            f'<span class="name">{html.escape(chapter.title)}</span>'
            '<span class="dots"></span>'
            f'<span class="page">{page_text(pages.get(chapter.id))}</span></a>'
        )
    # The index is at the back, after the last part.
    lines.append(
        '<a class="entry depth-0 index" href="#pdf-index">'
        '<span class="name">Índice alfabético</span><span class="dots"></span>'
        f'<span class="page">{page_text(pages.get("pdf-index"))}</span></a>'
    )
    lines.append("</nav>")
    return "\n".join(lines)


def index_html(entries: list[Entry], pages: dict[str, int]) -> str:
    lines = [
        heading("pdf-index", "Índice alfabético"),
        "<p>Las palabras de los glosarios de música y de Rust, las partes "
        "de la aplicación y los capítulos que las explican. Un número en "
        "<strong>negrita</strong> es la página a la que el glosario te "
        "envía para esa palabra; los demás son donde un capítulo la "
        "presenta o trata de ella.</p>",
        '<div class="pdf-index">',
    ]
    letter = None
    for e in entries:
        first = e.sort_key[:1].upper()
        first = first if first.isalpha() else "0–9"
        if first != letter:
            letter = first
            lines.append(f'<p class="letter">{letter}</p>')
        if e.see:
            lines.append(f'<p class="item">{e.label}, <em>véase</em> {e.see}</p>')
            continue
        # One number per page; where the word is explained wins.
        by_page: dict[int | None, tuple[str, bool]] = {}
        for id, explained in e.places:
            page = pages.get(id)
            if page not in by_page or (explained and not by_page[page][1]):
                by_page[page] = (id, explained)
        numbers = []
        for page in sorted(by_page, key=lambda p: -1 if p is None else p):
            id, explained = by_page[page]
            number = page_text(page)
            if explained:
                number = f"<strong>{number}</strong>"
            numbers.append(f'<a href="#{id}">{number}</a>')
        lines.append(f'<p class="item">{e.label}, {", ".join(numbers)}</p>')
    lines.append("</div>")
    # Chrome keeps where an id is in the PDF only when a link in the page
    # goes to it, and the list above has one link for each page, not for
    # each place. These links, never shown, keep every place, so the next
    # printing still knows where each one is.
    targets = "".join(f'<a href="#{id}"></a>' for e in entries for id, _ in e.places)
    lines.append(f"<div hidden>{targets}</div>")
    return "\n".join(lines)


def title_html(book: dict) -> str:
    title, _, subtitle = book["title"].partition(": ")
    authors = ", ".join(book.get("authors", []))
    return (
        '<section class="pdf-title-page">'
        f'<p class="title">{html.escape(title)}</p>'
        f'<p class="subtitle">{html.escape(subtitle)}</p>'
        f'<p class="authors">{html.escape(authors)}</p>'
        "</section>"
    )


def print_pdf(chrome: str, page: Path, out: Path) -> None:
    subprocess.run(
        [
            chrome,
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--no-pdf-header-footer",
            "--virtual-time-budget=30000",
            f"--print-to-pdf={out}",
            page.as_uri(),
        ],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    if not out.exists() or out.stat().st_size == 0:
        sys.exit(f"Chrome printed nothing to {out}")


def destinations(reader: PdfReader) -> dict[str, tuple[int, object]]:
    """Every place a link in the PDF can go to, by its id in the HTML: its
    page (from 0) and where on the page."""
    found = {}
    for name, dest in reader.named_destinations.items():
        id = urllib.parse.unquote(name.lstrip("/"))
        found[id] = (reader.get_destination_page_number(dest), dest)
    return found


def add_bookmarks(
    writer: PdfWriter,
    chapters: list[Chapter],
    dests: dict[str, tuple[int, object]],
) -> None:
    def add(title: str, id: str, parent=None, open_=False):
        page, dest = dests[id]
        top = dest.get("/Top")
        fit = Fit.xyz(left=dest.get("/Left"), top=top) if top is not None else Fit.fit()
        return writer.add_outline_item(title, page, parent=parent, fit=fit, is_open=open_)

    add("Contenido", "pdf-contents")
    part, part_item = None, None
    parents = []  # the bookmark of the chapter at each depth
    for chapter in chapters:
        if chapter.part != part:
            part = chapter.part
            part_item = add(part, chapter.id, open_=True)
        del parents[chapter.depth :]
        parent = parents[-1] if parents else part_item
        item = add(chapter.title, chapter.id, parent)
        for heading in chapter.headings[1:]:
            if heading.level == 2 and heading.id in dests:
                add(heading.title, heading.id, item)
        parents.append(item)
    add("Índice alfabético", "pdf-index")


def point_links_at_pages(writer: PdfWriter) -> None:
    """Gives every link inside the PDF its page and place, instead of a
    name to look up.

    Chrome writes a link to #step-9-the-band as "go to the place named
    step-9-the-band", and a table at the back of the PDF says where that
    is. Chrome's and Firefox's readers look the name up, but not every
    reader does: in some, those links do nothing. A link that says "go to
    page 543, this far down" works in all of them."""
    names = writer.root_object.get("/Dests")
    if names is None:
        return
    names = names.get_object()
    for page in writer.pages:
        for annotation in page.get("/Annots", []):
            link = annotation.get_object()
            name = link.get("/Dest")
            if not isinstance(name, (NameObject, TextStringObject)):
                continue
            place = names.get(name)
            if place is None:
                continue
            place = place.get_object()
            # A name can also lead to a dictionary that holds the place.
            if isinstance(place, DictionaryObject):
                place = place["/D"].get_object()
            link[NameObject("/Dest")] = ArrayObject(place)


def main() -> None:
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    # Chrome opens the page by its full address, file:///...
    built, out = Path(sys.argv[1]).resolve(), Path(sys.argv[2])
    here = Path(__file__).resolve().parent
    chrome = os.environ.get("CHROME", "google-chrome")
    # Chrome's PDFs make pypdf warn that they have more objects than they
    # say; they read fine.
    logging.getLogger("pypdf").setLevel(logging.ERROR)
    book = tomllib.loads((here / "book.toml").read_text(encoding="utf-8"))["book"]
    chapters = read_summary(here / book.get("src", "src") / "SUMMARY.md")
    words = [
        line.strip()
        for line in (here / "index-words.txt").read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.startswith("#")
    ]

    source = (built / "print.html").read_text(encoding="utf-8")
    start = source.index("<main>") + len("<main>")
    end = source.index("</main>")
    chunks = source[start:end].split(CHAPTER_BREAK)
    if len(chunks) != len(chapters):
        sys.exit(
            f"print.html has {len(chunks)} chapters, SUMMARY.md {len(chapters)}: "
            "has mdBook changed how it separates chapters?"
        )
    for chunk, chapter in zip(chunks, chapters, strict=True):
        chapter.headings = [
            Heading(int(level), html.unescape(id), plain(title))
            for level, id, title in HEADING.findall(chunk)
        ]
        if not chapter.headings:
            sys.exit(f"{chapter.title!r} has no heading to point at")
    chunks, entries = build_index(chunks, chapters, words)
    head_end = source.index("</head>")

    def write(pages: dict[str, int], path: Path) -> None:
        text = (
            source[:head_end]
            + STYLE
            + source[head_end:start]
            + title_html(book)
            + CHAPTER_BREAK
            + contents_html(chapters, pages)
            + CHAPTER_BREAK
            + CHAPTER_BREAK.join(chunks)
            + CHAPTER_BREAK
            + index_html(entries, pages)
            + source[end:]
        )
        path.write_text(text, encoding="utf-8")

    # Every place the contents, the bookmarks and the index point to.
    needed = ["pdf-contents", "pdf-index"] + [c.id for c in chapters]
    needed += [id for e in entries for id, _ in e.places]

    # The page is written next to print.html, so its styles, scripts and
    # pictures load from the same places.
    with tempfile.TemporaryDirectory() as tmp:
        html_path = built / "print-pdf.html"
        pdf_path = Path(tmp) / "book.pdf"
        pages: dict[str, int] = {}
        for _ in range(4):
            write(pages, html_path)
            print_pdf(chrome, html_path, pdf_path)
            reader = PdfReader(pdf_path)
            dests = destinations(reader)
            landed = {id: page for id, (page, _) in dests.items()}
            missing = [id for id in needed if id not in landed]
            if missing:
                sys.exit(f"no page for {', '.join(missing)}")
            if all(landed[id] == pages.get(id) for id in needed):
                break
            pages = landed
            print(f"printed {len(reader.pages)} pages, numbering them", file=sys.stderr)
        else:
            sys.exit("the page numbers keep moving")
        html_path.unlink()

        writer = PdfWriter(clone_from=reader)
        add_bookmarks(writer, chapters, dests)
        point_links_at_pages(writer)
        writer.page_mode = "/UseOutlines"
        out.parent.mkdir(parents=True, exist_ok=True)
        with open(out, "wb") as f:
            writer.write(f)
    print(f"{out}: {len(reader.pages)} pages, {len(entries)} index entries", file=sys.stderr)


if __name__ == "__main__":
    main()

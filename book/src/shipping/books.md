# How the books are built

Files: [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
[`book/book.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/book.toml),
[`book/print.css`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print.css),
[`book/print-pdf.py`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print-pdf.py)
and [`book/index-words.txt`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/index-words.txt)

Four books live in the repository, next to the code they describe:

| Folder | Book | On the web |
|---|---|---|
| `book/` | this book | <https://naelolaiz.github.io/ferrous-lineup/> |
| `book-es/` | this book in Spanish | <https://naelolaiz.github.io/ferrous-lineup/es/> |
| `manual/` | the user manual | <https://naelolaiz.github.io/ferrous-lineup/manual/> |
| `manual-es/` | the user manual in Spanish | <https://naelolaiz.github.io/ferrous-lineup/es/manual/> |

Each is a website and a PDF with contents, page numbers, an index and
bookmarks. A change to the app and the pages that describe it go in the
same pull request, or in a book pull request right after it, so the
books never describe a program that no longer exists.

## mdBook

[mdBook](https://rust-lang.github.io/mdBook/) turns a folder of Markdown
files into a website. `book.toml` holds the settings, and `src/SUMMARY.md`
the order of the chapters, which becomes the sidebar:

```text
# Summary

[Introduction](introduction.md)

[Building it from source](building.md)

# Part I: The shape of the program

- [Five crates, and what each one may know](program/crates.md)
```

A link on a line of its own, before the first part, is a chapter
without a number, like the introduction. A line starting with `#` is a
part's title, and each `- [...](...)` a numbered chapter (an indented
one would be a chapter inside the one above). `mdbook
build` writes the website into the book's `book/` folder, and `mdbook
serve --open` builds it, opens it, and builds it again each time a page
is saved: the way to read a page before pushing it.

`site-url` in `book.toml` is the path GitHub Pages serves the book from,
`/ferrous-lineup/` (or `/ferrous-lineup/es/`). Most pages find their
styles with relative links, but the "page not found" page can be shown
at any address, so it needs to know where the book starts.

A page that moves keeps its old address working. `book.toml` lists the
old paths, each with where its page went, and mdBook writes a small page
at the old path that sends the browser on:

```toml
[output.html.redirect]
"/step-9/band.html" = "../band/band.html"
```

## The workflow

The **Book** workflow has two jobs. **build** runs on every push to main
and every pull request that changes a book, a manual or the workflow
itself; **deploy** runs only after a push to main, or when started by
hand (**Actions > Book > Run workflow**).

```yaml
pull_request:
  paths:
    - "book/**"
    - "book-es/**"
    - "manual/**"
    - "manual-es/**"
    - ".github/workflows/book.yml"
```

`paths` is why a change to the app's code doesn't build the books, and a
book page doesn't wait for the app's tests. `pull_request` names no
branch, so a book page stacked on another pull request is built too.

1. **mdBook**, pinned to one version, is downloaded ready-built by
   `taiki-e/install-action`, which checks its checksum. The same version
   on every run builds the same books, however mdBook changes later.
2. **The websites:** `mdbook build` in each of the four folders.
3. **Fonts.** The books write rests and notes as music symbols (𝄾, ♩) and
   have a few emoji. A font without them prints an empty box, so the job
   installs FreeSerif and Noto Color Emoji.
4. **pypdf**, a Python library for PDF files, pinned too. Ubuntu's Python
   won't install packages for the whole system, so it goes into a
   **virtual environment**: a folder with a Python and packages of its
   own.
5. **The PDFs.** mdBook also writes `print.html`, every chapter on one
   page, which is what the printer button on the website opens. Each book
   is built a second time, into `pdf/`, in the light theme, since the
   website is dark but paper is white. mdBook reads a setting from an
   environment variable named after it, so
   `MDBOOK_OUTPUT__HTML__DEFAULT_THEME=light` overrides `default-theme`
   under `[output.html]` for that build only (`__` between the levels).
   Then `print-pdf.py` prints that page to a PDF.
6. **Uploads.** Each PDF is a download of the run, pull requests
   included. The four websites, with their PDFs, are put together into
   one site: the English book at the root, the Spanish one under `es/`,
   and each manual under `manual/` in its language.
7. **Publishing.** `deploy-pages` hands the site to GitHub Pages. That job
   alone asks for `pages: write` and `id-token: write`, to publish and to
   prove which workflow is publishing; every other job only reads the
   code. One deployment runs at a time (`concurrency: group: pages`): a
   newer one waits rather than cancel one halfway.

## Finding your way in the PDF

The print page, printed as it is, is hundreds of pages with no page
numbers, no contents and no index. On the website the sidebar and the
search box find a chapter; on paper nothing would. `print-pdf.py` adds
what a printed book needs:

| What | Made from |
|------|-----------|
| A title page and the **contents**, every chapter with the page it starts on | `SUMMARY.md` |
| A **page number** at the bottom of every page | `print.css` |
| An **index** at the back: words, and the pages that explain them | the [music](../reference/music-glossary.md) and [Rust](../reference/rust-glossary.md) glossaries, and `index-words.txt` |
| **Bookmarks**, the outline a PDF reader shows beside the pages | the parts, the chapters and their sections |

### Page numbers: CSS for paper

When a browser prints, it pours the page onto sheets of paper, and the
`@page` rule says what those sheets are:

```css
@page {
    size: A4;
    margin: 18mm 16mm;

    @bottom-center {
        content: counter(page);
        font-family: "Open Sans", sans-serif;
        font-size: 9pt;
        color: #555;
    }
}
```

Inside it, *margin boxes* name the spaces around the text:
`@bottom-center` is the middle of the bottom margin, and `counter(page)`
the number of the page being printed. The title page has none: the
script adds a rule for the first page only, `@page :first`.

### The contents: printing twice

The contents must say on which page each chapter starts before the book
is printed, but only printing decides where a chapter lands: the length
of every page before it depends on the fonts, the pictures and where each
line wraps. The script does what LaTeX does: it prints twice.

1. The first time, every page number in the contents and the index is
   `0000`.
2. The script reads in that PDF where each chapter and each word of the
   index landed.
3. The second time, the real numbers take the blanks' place. `0000` is as
   wide as any page number of the book, so no line grows, nothing moves,
   and the numbers are right. The script reads that PDF too, and if
   something did move, it prints again, up to four times.

Where a chapter landed is read through its links. mdBook gives every
heading an `id` and a link to itself, and when Chrome prints a page with
a link to `#the-chord-sheet`, it writes a **named destination** into the
PDF: the name, the page, and where on the page. pypdf reads them back:

```python
def destinations(reader: PdfReader) -> dict[str, tuple[int, object]]:
    found = {}
    for name, dest in reader.named_destinations.items():
        id = urllib.parse.unquote(name.lstrip("/"))
        found[id] = (reader.get_destination_page_number(dest), dest)
    return found
```

The names come back as `/the-chord-sheet`, with every letter outside
ASCII written as `%` codes (the `ö` of Kröpfl is `%C3%B6`), so `unquote`
turns them back into the `id`s of the HTML. Chrome only writes a
destination for an `id` some link goes to. Every heading links to itself,
and the contents and the index link to what they list; when the index
shows one number for two places on the same page, a hidden list of links
(`<div hidden>`) still points at both.

### The index

The books already have lists of words: their glossaries, tables with the
word in bold, what it means, and an "Explained in" link. The script
reads them from `print.html` and gives each word:

- the page its "Explained in" link goes to, in **bold**;
- every page where a chapter writes the word in bold, which is how these
  books introduce a word. Each such bold word gets an `id`, such as
  `pdf-ix-17`, for the index to link to.

A plural counts too, and a word that is all code only matches code: the
glossary's `windows`, a Rust method, is not the bold Windows of "Linux,
macOS and Windows". `index-words.txt` adds the words no glossary has,
mostly the parts of the app, with the chapters that explain them:

```text
Chord sheet: band/chord-sheet.md, ...
Modulation: see Key change
Function: not program/sync.md
```

- A place is a chapter, by the file `SUMMARY.md` links, or a section of
  it, by its heading's `id` after `#`.
- `see` sends the reader to another entry.
- `not` leaves out a chapter where the bold word means something else:
  there "function" is a Rust function, not what a chord does in a key.

A line that names a chapter or a section the book doesn't have stops the
script with an error, so a renamed heading fails the workflow instead of
sending readers to the wrong page.

### Bookmarks and links

Chrome writes no outline for this page, so the script adds one with
pypdf after the last printing: the parts at the top, each chapter under
its part, and a chapter's `##` sections under the chapter.

```python
writer = PdfWriter(clone_from=reader)
add_bookmarks(writer, chapters, dests)
point_links_at_pages(writer)
writer.page_mode = "/UseOutlines"
```

`/UseOutlines` asks the reader to open with the bookmarks shown.
`point_links_at_pages` fixes the links themselves. Chrome writes a link
as "go to the place named `the-chord-sheet`", and a table at the back of
the PDF gives each name a place. Chrome's and Firefox's readers look the
name up, but not every reader does, and in those such a link does
nothing. So the script looks every name up itself and writes the place
into the link, "go to page 543, this far down", which every reader
follows:

```python
link[NameObject("/Dest")] = ArrayObject(place)
```

## One script per language

The Spanish book has its own `print-pdf.py`, a copy with the Spanish
words: "Contenido" and "Índice alfabético" for the headings, `véase` and
`no` in `index-words.txt`, the Spanish glossaries, and two rules English
does not need. A word ending in *-ión* has its plural in *-iones*
(inversión, inversiones), and accents don't move a word in the index:
"Índice" sorts with the I's.

The manuals have no script of their own. Each is printed by its
language's script, given the manual's folder as a third argument, where
the script finds the manual's `book.toml` and `index-words.txt`.

## Printing a PDF yourself

Nothing here needs GitHub. To print the book's PDF as the workflow does,
you need mdBook, Python 3.11 or newer, and Chrome or Chromium:

```sh
python3 -m venv ~/pdf-tools
~/pdf-tools/bin/pip install pypdf
cd book
MDBOOK_OUTPUT__HTML__DEFAULT_THEME=light \
  MDBOOK_OUTPUT__HTML__PREFERRED_DARK_THEME=light \
  mdbook build --dest-dir pdf
~/pdf-tools/bin/python print-pdf.py pdf book/ferrous-lineup-book.pdf
```

If Chrome is not called `google-chrome` on your system, say where it is:
`CHROME=chromium ~/pdf-tools/bin/python print-pdf.py ...`.

The app is Rust; the books are built with whatever does each job most
simply: Markdown for the pages, YAML for the workflow, CSS for the paper,
Python and pypdf for the PDF.

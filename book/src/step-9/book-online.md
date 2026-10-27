# The book online and as a PDF

Files: [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
[`book/book.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/book.toml),
[`book/print.css`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print.css),
[`book/print-pdf.py`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print-pdf.py)
and [`book/index-words.txt`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/index-words.txt)

Until now this book lived only as Markdown files and as a website you
built yourself with `mdbook build`. It now lives in the `book/` folder
of the repository, next to the code it describes, and a workflow,
**Book**, turns it into two things anyone can open:

| What | Where |
|------|-------|
| The website | <https://naelolaiz.github.io/ferrous-lineup/> |
| One PDF with every chapter | <https://naelolaiz.github.io/ferrous-lineup/ferrous-lineup-book.pdf>, and as a download on every run in the **Actions** tab |

## A branch of its own, then a folder

For its first week online the book had a branch of its own, `book`,
that shared no history with `main`: its first commit was the book's
first chapter, not the app's first line of code. Git does not mind. A
branch is only a name for a commit, and nothing says all the commits in
a repository have to share a root.

That had one cost. GitHub shows a pull request as what its branch
changes since the last commit it shares with the branch it goes into.
A book branch and an app branch share no commit, so GitHub cannot
compare them: a book page could never be a pull request stacked on the
pull request for the change it describes, and pointing a book pull
request at an app branch made GitHub close it. So the book moved into
`book/` on `main`, with its history:

```sh
git merge -s ours --no-commit --allow-unrelated-histories origin/book
git read-tree --prefix=book/ -u origin/book
git commit
```

- `--allow-unrelated-histories` lets `git merge` join two histories with
  no commit in common; without it Git refuses.
- `-s ours` (the "ours" strategy) records the merge but keeps `main`'s
  files exactly as they are, so the book's `src/` does not land at the
  top of the repository.
- `git read-tree --prefix=book/ -u origin/book` then puts every file of
  the book under `book/`, in the index and (`-u`) in the folder.

The commit has two parents, the last commit of `main` and the last
commit of `book`, so every commit of the book is still there.
`git blame book/src/tour.md` names the commit that wrote each line, and
`git log --full-history -- src/tour.md` lists a chapter's commits from
before the move, under the path it had then.

## Step by step

The workflow has two jobs. **build** runs on every push to `main` and
every pull request that changes something in `book/` (or the workflow
itself); **deploy** runs only after a push, or when you start the
workflow by hand (**Actions > Book > Run workflow**).

```yaml
on:
  push:
    branches: [main]
    paths:
      - "book/**"
      - ".github/workflows/book.yml"
  pull_request:
    paths:
      - "book/**"
      - ".github/workflows/book.yml"
  workflow_dispatch:
```

`paths` is why a change to the app's code doesn't build the book, and a
book page doesn't wait for the app's tests. `pull_request` names no
branch, so a book page stacked on another pull request is built too,
whatever it goes into. The build job runs its commands in `book/`
(`defaults: run: working-directory: book`), as if the book were the
whole repository; paths given to actions (`with: path:`) still start at
the top, so they say `book/book`.

1. **mdBook.** `taiki-e/install-action` downloads a released mdBook,
   pinned to one version, so the book builds the same way on every run
   however mdBook changes later. `mdbook build` writes the website to
   `book/book/`, exactly as on your computer.
2. **The PDF.** mdBook also writes `print.html`, every chapter on one
   page; it is what the printer button at the top right of the website
   opens. The workflow builds the book a second time, into `pdf/`, in the
   light theme: the website stays dark blue, but a PDF should be black
   on white. mdBook reads any setting from an environment variable
   instead of `book.toml` when it is named after the setting:
   `MDBOOK_OUTPUT__HTML__DEFAULT_THEME` is `default-theme` under
   `[output.html]`, with `__` between the levels and `_` for `-`. Then
   `print-pdf.py` adds a title page, the contents and an index to that
   page, and has Chrome, which GitHub's machines already have, open it
   without a window (`--headless`) and print it to a file
   (`--print-to-pdf`), the way **Print** does in a browser, without the
   date and the address at the top and bottom of each page
   (`--no-pdf-header-footer`). Last, it adds the bookmarks
   ([Finding your way in the PDF](#finding-your-way-in-the-pdf) below).
3. **Fonts.** The book writes rests and notes as music symbols (𝄾, ♩)
   and has a few emoji. A font without them prints an empty box, so the
   job installs FreeSerif and Noto Color Emoji first.
4. **Two uploads.** The PDF is uploaded on its own, as the run's
   download. Then `upload-pages-artifact` packs `book/book/`, with the
   PDF copied into it, as the website.
5. **Publishing.** `deploy-pages` hands that pack to GitHub Pages. The
   job asks for two permissions only it needs: `pages: write` to
   publish and `id-token: write` to prove to Pages which workflow is
   publishing. Every other job only reads the code
   (`permissions: contents: read` at the top). Only one deployment runs
   at a time (`concurrency: group: pages` on this job): a newer one waits
   instead of cancelling one halfway. Builds of pull requests don't wait.

`book/print.css` is the book's only style of its own on the website. It
says what a printed page is, A4 with a margin around the text and the
page number at the bottom, so the PDF and the website's printer button
make the same pages:

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

## Finding your way in the PDF

The first PDF was the website's print page, printed: over a thousand
pages with no page numbers, no contents and no index. On the website the
sidebar and the search box find a chapter; in a PDF reader, or on paper,
nothing did. `book/print-pdf.py`, a Python script, now prints the PDF and
adds what a printed book needs to find things in it:

| What | Made from |
|------|-----------|
| A title page and the **contents**: every chapter with the page it starts on | `SUMMARY.md`, the list the sidebar shows |
| A **page number** at the bottom of every page | `print.css` |
| An **index** at the back: words, and the pages that explain them | the [music](../music-glossary.md) and [Rust](../glossary.md) glossaries, and `index-words.txt` |
| **Bookmarks**, the outline a PDF reader shows beside the pages | the parts, the chapters and their sections |

### Page numbers: CSS for paper

When a browser prints a web page, it pours the text onto pages, and the
`@page` rule says what those pages are. Inside it, *margin boxes* name
the spaces around the text: `@bottom-center` is the middle of the bottom
margin, and `counter(page)` is the number of the page being printed. That
is the whole of it, in the `print.css` above. Chrome draws margin boxes
since version 131. The title page has no number: the script adds
`@page :first { @bottom-center { content: none; } }`, a rule for the
first page only.

### The contents: printing twice

The contents have to say "Step 9 starts on page 543" before the page is
printed, but only printing decides where Step 9 lands: the length of
every chapter before it depends on the fonts, the pictures and where each
line wraps. The script does what LaTeX, the typesetting program many
books are written in, does: it prints twice.

1. The first time, every page number in the contents and the index is
   `0000`.
2. The script reads in that PDF where each chapter, and each word of the
   index, landed.
3. The second time, the real numbers take the place of the blanks.
   `0000` is as wide as any page number in the book, so no line gets
   longer, nothing moves, and the numbers are right. The script reads
   the second PDF too, and if something did move, it prints a third
   time.

How do you read in a PDF where a chapter landed? Through its links.
mdBook gives every heading an `id` and a link to itself:

```html
<h1 id="step-9-the-band"><a class="header" href="#step-9-the-band">Step 9: The band</a></h1>
```

When Chrome prints a page with a link to `#step-9-the-band`, it writes a
*named destination* into the PDF: the name, the page, and where on the
page. That is how a click on a link in a PDF knows where to go, and
pypdf, a Python library for PDF files, reads them back:

```python
def destinations(reader: PdfReader) -> dict[str, tuple[int, object]]:
    found = {}
    for name, dest in reader.named_destinations.items():
        id = urllib.parse.unquote(name.lstrip("/"))
        found[id] = (reader.get_destination_page_number(dest), dest)
    return found
```

The names come back as `/step-9-the-band`, and with every letter outside
ASCII written as `%` codes (the `ö` of `kröpfls` is `%C3%B6`), so
`unquote` turns them back into the `id`s of the HTML.

Chrome only writes a destination for an `id` that some link in the page
goes to. Every heading links to itself, and the contents and the index
link to what they list; when the index shows one number for two places on
the same page, a hidden list of links (`<div hidden>`) still points at
both, so the next printing knows where each one is.

### The index

An index lists words with the pages about them. The book already has two
lists of words, its glossaries: tables with the word in bold, what it
means, and an "Explained in" link. The script reads them from
`print.html` and gives each word:

- the page its "Explained in" link goes to, in **bold**;
- every page where a chapter writes the word in bold. That is how this
  book introduces a word (`A **closure** is...` in the Markdown), so
  those are the pages where a chapter explains it. Each of those bold words gets an `id`,
  such as `pdf-ix-17`, for the index to link to.

The plural counts too: a bold "inversions" is a place for "Inversion". A
word that is all code only matches code: the glossary's `windows`, a
Rust method, is not the bold Windows of "Linux, macOS and Windows".

`book/index-words.txt` adds the words that are in no glossary, mostly the
parts of the app, with the chapters that explain them:

```text
Chord sheet: step-9/chord-sheet.md, step-9/text-chord-sheets.md, ...
Key marker: step-8/key-changes.md#a-key-marker
Modulation: see Key change
Function: not step-5/diff.md
```

- A place is a chapter, by the file `SUMMARY.md` links, or a section of
  it, by its heading's `id` after `#`.
- `see` sends the reader to another entry.
- `not` leaves a chapter out where its bold word means something else. In
  [Telling the audio thread what changed](../step-5/diff.md), the bold
  "function" is a Rust function, not what a chord does in the key.

A line that names a chapter or a section the book doesn't have stops the
script with an error, so a renamed heading fails the Book workflow
instead of sending readers to the wrong page.

### Bookmarks

Most PDF readers show an *outline* beside the pages: a tree of titles to
click. Chrome doesn't write one for this page, so the script adds it with
pypdf after the last printing:

```python
writer = PdfWriter(clone_from=reader)
add_bookmarks(writer, chapters, dests)
writer.page_mode = "/UseOutlines"
```

`add_outline_item` takes a title, a page, the bookmark it goes under and
where on the page to land (the destination of the heading). The parts
are at the top, each chapter under its part or its step, and a chapter's
sections, its `##` headings, under the chapter. `/UseOutlines` asks the
reader to show the panel when it opens the PDF.

### In the workflow

```yaml
- name: Install pypdf
  run: |
    python3 -m venv "$RUNNER_TEMP/pdf-tools"
    "$RUNNER_TEMP/pdf-tools/bin/pip" install --quiet pypdf==6.17.0
```

Ubuntu's Python won't install packages for the whole system, where they
would mix with the ones Ubuntu itself installs, so pypdf goes into a
*virtual environment*: a folder with a Python and packages of its own.
Its version is pinned, like mdBook's. The printing step runs the script
with that Python:

```yaml
mdbook build --dest-dir pdf
"$RUNNER_TEMP/pdf-tools/bin/python" print-pdf.py pdf book/ferrous-lineup-book.pdf
```

The app is Rust; the book is built with whatever does each job most
simply: YAML for the workflow, Python and pypdf for the PDF.

## Before the first push

Two settings on GitHub, made once:

- **Settings > Pages > Source**: GitHub Actions. Without it,
  `configure-pages` stops the run and says Pages is not set up.
- **Settings > Environments > github-pages > Deployment branches and
  tags**: turning Pages on creates this environment, and it lets the
  default branch, `main`, publish. While the book had its own branch,
  `book` had to be added there too; now `main` is enough.

`site-url` in `book.toml` is the path Pages serves the book from,
`/ferrous-lineup/`. Most pages find their styles with relative links,
but the "page not found" page can be shown for any address, so it needs
to know where the book starts.

## Reading it before you push

Nothing of this needs GitHub to try:

```sh
cd book
mdbook serve --open
```

builds the website and opens it, and builds it again each time you save
a chapter. **Print** in the browser on any page, or on the printer
button, shows the PDF's pages as they will look.

To print the whole PDF as the workflow does, with its contents, index and
bookmarks, you need Python 3.11 or newer and Chrome or Chromium:

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

## Exercises

1. In `book/`, run `MDBOOK_OUTPUT__HTML__DEFAULT_THEME=ayu mdbook build`
   and open `book/index.html`. Which theme does it show? Is `book.toml`
   changed?
2. Change `size: A4` to `size: letter` in `print.css` and print a page
   from the browser. What changes, and what stays?
3. Open a pull request that only changes a file in `crates/`. Which
   workflows run, and which line of `book.yml` keeps the Book workflow
   from running?
4. Run `git log --oneline --graph -5` on the commit that moved the book
   (its message starts "Bring the book's history"). How many parents does
   it have, and where does each line of the graph come from?
5. Add `Metronome: step-9/practice.md#where-the-clicks-come-from` to
   `index-words.txt` and print the PDF. Where does "Metronome" land in the
   index, and does its page number take you to the section? Then misspell
   the section and print again: what does the script say?
6. Change `BLANK` in `print-pdf.py` from `"0000"` to `"0"`. How many times
   does the script print the book now, and why can a narrower blank make
   the pages move?

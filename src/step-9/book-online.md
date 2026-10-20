# The book online and as a PDF

Files: [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/book/.github/workflows/book.yml),
[`book.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/book/book.toml)
and [`print.css`](https://github.com/naelolaiz/ferrous-lineup/blob/book/print.css)

Until now this book lived only as Markdown files and as a website you
built yourself with `mdbook build`. It now has a home of its own, the
`book` branch of the repository, and a workflow, **Book**, that turns it
into two things anyone can open:

| What | Where |
|------|-------|
| The website | <https://naelolaiz.github.io/ferrous-lineup/> |
| One PDF with every chapter | <https://naelolaiz.github.io/ferrous-lineup/ferrous-lineup-book.pdf>, and as a download on every run in the **Actions** tab |

The `book` branch shares no history with `main`: its first commit is the
book's first chapter, not the app's first line of code. Git does not
mind. A branch is only a name for a commit, and nothing says all the
commits in a repository have to share a root. So the app and its book
sit in one repository, each with its own history, and a push to one never
runs the other's workflows: GitHub reads the workflows from the commit
that was pushed, and the commits on `book` only have `book.yml`.

## Step by step

The workflow has two jobs. **build** runs on every push to `book` and
on every pull request into it; **deploy** runs only after a push.

1. **mdBook.** `taiki-e/install-action` downloads a released mdBook,
   pinned to one version, so the book builds the same way on every run
   however mdBook changes later. `mdbook build` writes the website to
   `book/`, exactly as on your computer.
2. **The PDF.** mdBook also writes `print.html`, every chapter on one
   page; it is what the printer button at the top right of the website
   opens. The workflow builds the book a second time, into `pdf/`, in the
   light theme: the website stays dark blue, but a PDF should be black
   on white. mdBook reads any setting from an environment variable
   instead of `book.toml` when it is named after the setting:
   `MDBOOK_OUTPUT__HTML__DEFAULT_THEME` is `default-theme` under
   `[output.html]`, with `__` between the levels and `_` for `-`. Then
   Chrome, which GitHub's machines already have, opens that page without
   a window (`--headless`) and prints it to a file
   (`--print-to-pdf`), the way **Print** does in a browser, without the
   date and the address at the top and bottom of each page
   (`--no-pdf-header-footer`).
3. **Fonts.** The book writes rests and notes as music symbols (𝄾, ♩)
   and has a few emoji. A font without them prints an empty box, so the
   job installs FreeSerif and Noto Color Emoji first.
4. **Two uploads.** The PDF is uploaded on its own, as the run's
   download. Then `upload-pages-artifact` packs `book/`, with the PDF
   copied into it, as the website.
5. **Publishing.** `deploy-pages` hands that pack to GitHub Pages. The
   job asks for two permissions only it needs: `pages: write` to
   publish and `id-token: write` to prove to Pages which workflow is
   publishing. Every other job only reads the code
   (`permissions: contents: read` at the top).

`print.css` is the book's only style of its own. It says what a printed
page is, A4 with a margin around the text, so the PDF and the
website's printer button make the same pages:

```css
@page {
    size: A4;
    margin: 18mm 16mm;
}
```

## Before the first push

Two settings on GitHub, made once:

- **Settings > Pages > Source**: GitHub Actions. Without it,
  `configure-pages` stops the run and says Pages is not set up.
- **Settings > Environments > github-pages > Deployment branches and
  tags**: add `book`. Turning Pages on creates this environment, and it
  only lets the default branch (`main`) publish; a push to `book` would
  build the book and then be refused.

`site-url` in `book.toml` is the path Pages serves the book from,
`/ferrous-lineup/`. Most pages find their styles with relative links,
but the "page not found" page can be shown for any address, so it needs
to know where the book starts.

## Reading it before you push

Nothing of this needs GitHub to try:

```sh
mdbook serve --open
```

builds the website and opens it, and builds it again each time you save
a chapter. **Print** in the browser on any page, or on the printer
button, shows the PDF's pages as they will look.

## Exercises

1. Run `MDBOOK_OUTPUT__HTML__DEFAULT_THEME=ayu mdbook build` and open
   `book/index.html`. Which theme does it show? Is `book.toml` changed?
2. Change `size: A4` to `size: letter` in `print.css` and print a page
   from the browser. What changes, and what stays?

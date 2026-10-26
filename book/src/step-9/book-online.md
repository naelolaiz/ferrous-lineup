# The book online and as a PDF

Files: [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
[`book/book.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/book.toml)
and [`book/print.css`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print.css)

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
   Chrome, which GitHub's machines already have, opens that page without
   a window (`--headless`) and prints it to a file
   (`--print-to-pdf`), the way **Print** does in a browser, without the
   date and the address at the top and bottom of each page
   (`--no-pdf-header-footer`).
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

`book/print.css` is the book's only style of its own. It says what a printed
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

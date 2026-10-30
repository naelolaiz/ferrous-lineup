# Release packages

Files: [`.github/workflows/release.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/release.yml)
and the folders of [`packaging/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/packaging)

Most people who use Ferrous Lineup will never install Rust. The
**Release** workflow builds the program for four kinds of computer and
packs seven downloads:

| Package | System |
|---------|--------|
| `ferrous-lineup-…-linux-x86_64.AppImage` | Linux on a PC (Intel or AMD): one file, nothing to unpack |
| `ferrous-lineup-…-linux-x86_64.tar.gz` | Linux on a PC, as a plain folder |
| `ferrous-lineup_…_amd64.deb` | Debian, Ubuntu and their derivatives |
| `ferrous-lineup-…-windows-x86_64-setup.exe` | Windows on a PC, with an uninstaller and the `.lineup` file association |
| `ferrous-lineup-…-windows-x86_64.zip` | Windows on a PC, portable: unzip and run |
| `ferrous-lineup-…-macos-arm64.dmg` | Macs with Apple Silicon (M1 and later) |
| `ferrous-lineup-…-macos-x86_64.dmg` | Macs with an Intel processor |

The `…` is the version for a release (`v0.4.0`), or the first seven
characters of the commit for any other build (`1a2b3c4`), so a download
always says what it is. How to open each one on its system is in the
manual's [Installing
it](https://naelolaiz.github.io/ferrous-lineup/manual/installing.html);
this chapter is about how they are made.

## Where the downloads are

- **Every pipeline** runs the workflow: each pull request, each push to
  main, and each run by hand (**Actions > Release > Run workflow**
  builds any branch). The packages are the run's **artifacts**, at the
  bottom of its page. Artifacts need a GitHub login to download, and
  GitHub deletes them after 90 days. So a pull request can be tried on a
  Mac or on Windows before it is merged.
- **A tag** that starts with `v` also publishes the packages on the
  repository's **Releases** page, with a `SHA256SUMS.txt` beside them. A
  release stays there for good, and anyone can download it.

## Cutting a release

A release is a tag: a name stuck on one commit. Making version 0.5.0
takes a pull request and a tag.

**The version pull request** changes:

- the version in the root `Cargo.toml`, under `[workspace.package]`,
  which every crate takes with `version.workspace = true`;
- `Cargo.lock`, which lists the version of every crate, ours too (one
  `cargo build` updates it);
- the release notes in the AppStream file,
  `packaging/linux/io.github.naelolaiz.FerrousLineup.metainfo.xml`, in
  English and Spanish, which software centres show;
- the default version in the Windows installer's script,
  `packaging/windows/ferrous-lineup.iss`;
- the version in the AUR recipes and the tag in the Flathub manifest.

**The tag**, once that is merged:

```sh
git fetch origin
git tag -a v0.5.0 -m "Ferrous Lineup 0.5.0" origin/main
git push origin v0.5.0
```

The push starts the workflow. When all its build jobs have passed, the
release appears, called "Ferrous Lineup v0.5.0", with notes that list
the pull requests merged since the last release; the notes can then be
edited on GitHub. A last small pull request pins what only exists once
the tag does: the checksum of GitHub's archive of the tag in the AUR
recipe, and the tag's commit in the Flathub manifest.

Two details:

- A tag with a dash, such as `v0.5.0-beta.1`, makes a **pre-release**,
  never shown as the latest. That is how [semantic
  versioning](https://semver.org) writes a version that is not final
  yet, and Cargo reads versions the same way.
- If the tag and `Cargo.toml` disagree, the release is still made, with a
  warning on the run's page. The tag decides the file names, but the
  program's own **Help > About** says what `Cargo.toml` said.

## Targets: which computer a program is for

A compiled program is machine code for one kind of processor, in the
file format of one operating system. Rust names each combination with a
**target triple**, architecture-vendor-system(-environment):

| Target | Processor | System |
|--------|-----------|--------|
| `x86_64-unknown-linux-gnu` | 64-bit Intel or AMD | Linux, with the GNU C library |
| `x86_64-pc-windows-msvc` | 64-bit Intel or AMD | Windows, with Microsoft's tools |
| `aarch64-apple-darwin` | 64-bit ARM (Apple Silicon) | macOS ("Darwin" is its core) |
| `x86_64-apple-darwin` | 64-bit Intel | macOS |

`rustc --version --verbose` shows your own computer's target on its
`host:` line. `cargo build --target <triple>` builds for another one,
once `rustup target add <triple>` has fetched the standard library for
it, and puts the program in `target/<triple>/release/`, so builds for
different targets never mix.

In the workflow each target is a line of a matrix (see [One job, three
machines](ci.md#one-job-three-machines-the-matrix)):

```yaml
- name: Linux x86_64
  os: ubuntu-22.04
  target: x86_64-unknown-linux-gnu
  platform: linux-x86_64
  appimage: true
```

and the build is

```sh
cargo build --release --locked --target ${{ matrix.target }}
```

`--locked` builds exactly the library versions in `Cargo.lock`, and
stops with an error if the file would have to change. A release should
be built from what was tested, not from whatever versions are newest on
the day of the tag. That is also why the version pull request must
commit `Cargo.lock`: with the old file, `--locked` refuses to build.

### The Intel Mac build, made on an Apple Silicon Mac

GitHub's macOS machines are Apple Silicon. Building for another
processor than the one you build on is **cross-compiling**, and on a Mac
it is easy: Apple's compilers and system libraries cover both
processors. So the Intel job runs on the same kind of machine as the
Apple Silicon one; only its target differs. The bits of C in the build,
such as the cryptography in `ring` (for downloading the SoundFont over
HTTPS), are compiled by the `cc` crate, which passes the target on to
Apple's C compiler. The workflow can't run the Intel program, which is
fine: the tests run in CI, and the code is the same.

### Why Linux builds on an older Ubuntu

A Linux program calls the C library, glibc, for files, memory and
threads. Each glibc function carries a version, and a program records
the newest one it used: built on Ubuntu 24.04, Ferrous Lineup would ask
for `GLIBC_2.39`. An older glibc refuses to start it ("version
`GLIBC_2.39' not found"), while a newer one runs it happily. So the
Linux build runs on the oldest Ubuntu GitHub offers, 22.04, with glibc
2.35, and the program runs on Ubuntu 22.04, Debian 12 and anything newer.

`ldd` lists the libraries a program loads when it starts:

```sh
$ ldd target/release/ferrous-lineup
	libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6
```

Only ALSA is there. The window libraries (Wayland or X11, xkbcommon for
the keyboard, OpenGL) are not missing: the crates open them while the
program runs, with `dlopen`, and use whichever the desktop has. That is
why one Linux build works on both Wayland and X11 desktops.

### The C runtime inside the Windows program

Rust programs on Windows use Microsoft's C runtime for a few basics. By
default it is a separate DLL that comes with the "Visual C++
Redistributable", which most Windows computers have, but not all. The
Windows line of the matrix adds a compiler flag:

```yaml
rustflags: -C target-feature=+crt-static
```

`crt-static` copies the runtime into the `.exe` (which grows a little),
so the program starts on a Windows that never installed anything. The
`RUSTFLAGS` environment variable passes flags to every `rustc` Cargo
runs; on the other lines it is empty.

## Packing

For Linux and Windows, a **Package** step copies the program, the README
and the licence into a folder named after the label, and packs it:

```bash
if [[ "$GITHUB_REF_TYPE" == tag ]]; then
  label="$GITHUB_REF_NAME"
else
  label="${COMMIT::7}"
fi
name="ferrous-lineup-$label-${{ matrix.platform }}"
```

`${COMMIT::7}` is Bash for "the first seven characters". `COMMIT` is
`${{ github.event.pull_request.head.sha || github.sha }}`: for a pull
request GitHub builds a merge commit of its own, which nobody ever sees,
so the label takes the branch's latest commit instead; `||` picks the
first value that isn't empty. Windows gets a `.zip`, which it opens by
itself. Linux gets a `.tar.gz`, because a tar file keeps the program's
"executable" permission.

For macOS, **Package for macOS** builds a real `Ferrous Lineup.app`:
the program in `Contents/MacOS`, `Info.plist` and the icon from
`packaging/macos/`, the version from `Cargo.toml` written into the plist.
It signs the app **ad-hoc** (`codesign --force --sign -`), puts it in a
folder with a link to `/Applications`, and `hdiutil` makes the `.dmg`,
so people drag the app onto Applications.

The plist also declares the `.lineup` **document type**: a
`CFBundleDocumentTypes` entry naming the type the app opens, and a
`UTExportedTypeDeclarations` entry that defines that type,
`io.github.naelolaiz.FerrousLineup.project`, as the files ending in
`.lineup`. With both, the Finder opens a project with the app. On Linux
and Windows the file manager starts the program with the file's path
on the command line, which `main` reads. macOS starts it with no
arguments, and sends the files in an Apple event, "open documents"
(`aevt`/`odoc`), which also reaches an app already running. AppKit's
own handler for it asks the application's delegate to open the files;
the delegate belongs to winit, the window library, and opens nothing.
So `open_files.rs` sets its own handler, at the moment Apple's rules
give for it, when the application is about to finish launching (after
AppKit sets its handler, before the event of the launch arrives). The
handler only collects the paths:

```rust
#[unsafe(method(openDocuments:withReplyEvent:))]
fn open_documents(
    &self,
    event: &NSAppleEventDescriptor,
    _reply: &NSAppleEventDescriptor,
) {
    // SAFETY: `paramDescriptorForKeyword:` takes a four-letter
    // code and gives a descriptor or nil.
    let files: Option<Retained<NSAppleEventDescriptor>> =
        unsafe { msg_send![event, paramDescriptorForKeyword: DIRECT_OBJECT] };
    let Some(files) = files else {
        return;
    };
    // A list of files, counted from 1, or a single one.
    let count = files.numberOfItems();
    let items: Vec<Retained<NSAppleEventDescriptor>> = if count == 0 {
        vec![files]
    } else {
        (1..=count)
            .filter_map(|index| files.descriptorAtIndex(index))
            .collect()
    };
    super::arrived(
        items
            .iter()
            .filter_map(|item| item.fileURLValue())
            .filter_map(|url| url.to_file_path()),
    );
}
```

`objc2` calls Objective-C from Rust: `define_class!` makes a class
whose methods AppKit can call, and `msg_send!` sends a message by name,
which the compiler cannot check, hence `unsafe`. `arrived` keeps the
paths in a `Mutex<Vec<PathBuf>>` and asks the window to repaint;
`handle_dropped_files` takes them on the next frame and opens the file
as if it had been dropped on the window. All of it is under
`#[cfg(target_os = "macos")]`, so on Linux and Windows `listen` does
nothing.

Every package is uploaded with `archive: false`:

```yaml
- uses: actions/upload-artifact@v7
  with:
    name: ${{ env.NAME }}
    path: ${{ env.PACKAGE }}
    archive: false
```

By default an artifact is zipped on upload, so a `.tar.gz` would
download as a zip holding a `.tar.gz`. With `archive: false` the file is
stored as it is, and the artifact is named after the file.

### The AppImage

An **AppImage** is a program and what it needs, packed into one file
that starts like a program. The file is two things glued together: a
small starter program, the **runtime**, followed by a compressed disk
image of a folder, the **AppDir**. Started, the runtime mounts the image
as a folder (with FUSE, which lets a normal program provide a file
system) and starts the program inside. Nothing is installed, so deleting
the file uninstalls it. The AppDir is laid out like a system's `/usr`:

```text
AppDir/
├── AppRun -> usr/bin/ferrous-lineup          what the runtime starts
├── io.github.naelolaiz.FerrousLineup.desktop -> usr/share/…  the menu entry
├── io.github.naelolaiz.FerrousLineup.svg -> usr/share/…      the icon
└── usr/
    ├── bin/ferrous-lineup
    ├── lib/libxkbcommon.so.0, libxkbcommon-x11.so.0, libxcb-xkb.so.1
    └── share/applications/, icons/, mime/, metainfo/
```

The **desktop entry**
([`packaging/linux/io.github.naelolaiz.FerrousLineup.desktop`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/linux/io.github.naelolaiz.FerrousLineup.desktop))
is the text file every Linux menu reads: the app's name, its icon, its
category and how to start it (`Exec=ferrous-lineup %f`, where `%f` is a
file to open). The MIME type file teaches the desktop what a `.lineup`
file is, and the AppStream **metainfo** file describes the app and its
releases for software centres. The workflow checks all three with
`desktop-file-validate` and `xmllint`, because a typo in them fails
silently on a desktop.

`appimagetool` turns the AppDir into the AppImage. It and the runtime
are downloaded pinned to one version and checked against the SHA-256
written in the workflow:

```bash
echo "$RUNTIME_SHA256  runtime" | sha256sum --check
```

If someone replaced the file on GitHub, the checksum would differ and
the build would stop, rather than ship whatever the file had become.

#### Bringing a library along

On a system without `libxkbcommon-x11`, the program stops before it
opens a window. `ldd` doesn't list that library, because winit opens it
with `dlopen` when the window is made. A library opened this way is
searched for like any other: in the folders of `LD_LIBRARY_PATH`, then
the **RUNPATH** written in the program, then the system's folders. So
the workflow copies the three keyboard libraries into `AppDir/usr/lib`
and writes a RUNPATH with `patchelf`:

```bash
patchelf --set-rpath '$ORIGIN/../lib' AppDir/usr/bin/ferrous-lineup
patchelf --set-rpath '$ORIGIN' AppDir/usr/lib/libxkbcommon-x11.so.0
```

`$ORIGIN` means "the folder this file is in", worked out when the
program starts, so the path holds wherever the AppImage is mounted. The
single quotes keep the shell from replacing `$ORIGIN` with an empty
variable. What the AppImage does **not** carry: glibc, ALSA and the
graphics drivers. Those must match the computer, and the AppImage
community keeps a list of libraries an AppImage should never bring.

### The Debian package

The `.deb` is built by `cargo-deb`, pinned to one version, from the same
program and the same desktop entry, icon, MIME type and AppStream file:

```sh
cargo deb -p lineup-app --no-build --target x86_64-unknown-linux-gnu
```

`--no-build` takes the program the matrix just built. The
`[package.metadata.deb]` table in `crates/lineup-app/Cargo.toml` lists
the files and what the package depends on (ALSA, Wayland, xkbcommon,
the desktop portal and the tools that register menu entries, icons and
MIME types), so `apt` installs those too.

### The Windows installer

The Setup program is made by **Inno Setup**, which GitHub's Windows
machines already have, from the script
[`packaging/windows/ferrous-lineup.iss`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/windows/ferrous-lineup.iss).
The workflow passes it the version and the label:

```bash
"$iscc" /DAppVersion="$version" /DBuildLabel="$LABEL" packaging/windows/ferrous-lineup.iss
```

The script installs the program for the user who runs it, with no
administrator rights needed, and adds a Start-menu entry, an optional
desktop shortcut, an uninstaller and the `.lineup` file association.

## Publishing: a job that waits for the others

The second job, `release`, runs only for tags, and only once every build
job has passed:

```yaml
release:
  name: Publish the release
  if: github.ref_type == 'tag'
  needs: build
```

`needs: build` waits for every line of the matrix. If one build fails,
there is no release at all, rather than a release missing Windows.

The job's `permissions` are about the **token** every run gets,
`GITHUB_TOKEN`, a password that expires when the job ends. At the top of
the file, `permissions: contents: read` lets every job read the code and
nothing else, so a build step cannot change the repository. Only this
job asks for `contents: write`, which a release needs.

It downloads the packages, writes their checksums with `sha256sum`, and
hands everything to `gh`, GitHub's command-line tool:

```bash
if gh release view "$TAG" > /dev/null 2>&1; then
  gh release upload "$TAG" release/* --clobber
else
```

- `gh release view` succeeds when the release exists already, made on
  the website or by an earlier run. Then the files are uploaded to it,
  and `--clobber` replaces files of the same name.
- Otherwise `gh release create` makes it, with `--generate-notes` for
  the notes and `--verify-tag`, which stops if the tag isn't on GitHub
  instead of creating a new tag on the newest commit.
- `--prerelease` is added when the tag has a dash.

A **checksum** is a 64-character fingerprint of a file's bytes: change
one byte and the fingerprint is completely different. `SHA256SUMS.txt`
lets anyone check that a download arrived whole.

## Testing a workflow before it is merged

A release workflow only does its real work on a tag, which is a bad
moment to find a typo. Two things catch mistakes earlier:

- Every pull request runs it, so a change that breaks the Windows or the
  Mac build shows on its pull request. Only the `release` job is
  skipped, since a pull request is not a tag.
- [actionlint](https://github.com/rhysd/actionlint) checks workflow
  files, and, with [ShellCheck](https://www.shellcheck.net), the shell
  scripts inside them:

  ```sh
  pip install actionlint-py shellcheck-py
  actionlint .github/workflows/release.yml
  ```

`concurrency` keeps one run per branch: when a pull request gets a new
push, the older run is cancelled, since the newer one contains it. Tags
are never cancelled.

## Other ways in: the AUR and Flathub

Two more recipes live in `packaging/`, for package managers that build
the program themselves from the source:

- `packaging/aur/` holds two **PKGBUILD**s for Arch Linux's user
  repository: `ferrous-lineup`, built from a release's tag, and
  `ferrous-lineup-git`, built from main.
- `packaging/flathub/` holds a **Flatpak** manifest for Flathub, with
  `cargo-sources.json`, every crate of `Cargo.lock` by URL and checksum:
  Flathub builds without a network, so every download must be listed in
  advance. `generate-sources.py` writes that file again after
  `Cargo.lock` changes.

Both install the same desktop entry, icon, MIME type and AppStream file
as the AppImage and the `.deb`. Their READMEs say how to publish them.

## What the packages don't do yet

- The programs are not fully **signed**. The macOS app has an ad-hoc
  signature, which Apple Silicon requires, but is not **notarized**: that
  needs a paid Apple Developer ID, so Gatekeeper asks the first time. The
  Windows programs are unsigned, so SmartScreen warns. The manual's
  [Installing
  it](https://naelolaiz.github.io/ferrous-lineup/manual/installing.html)
  shows how to get past each warning.

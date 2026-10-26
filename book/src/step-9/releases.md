# Release builds: a download for every system

File: [`.github/workflows/release.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/release.yml)

Until now, the only way to try Ferrous Lineup was to install Rust and
build it. The CI workflow did attach the program it built to each run
([Proving it: the CI matrix](../step-4/platforms.md#proving-it-the-ci-matrix)),
but only as a bare file, with no Intel Mac build, and nothing a person
without a GitHub account could find. A second workflow, **Release**,
now runs four build jobs (Linux, Windows, and macOS for Apple Silicon
and Intel) and packs seven downloads:

| Package | System |
|---------|--------|
| `ferrous-lineup-…-linux-x86_64.AppImage` | Linux on a PC (Intel or AMD): one file, nothing to unpack |
| `ferrous-lineup-…-linux-x86_64.tar.gz` | Linux on a PC, as a plain folder |
| `ferrous-lineup_…_amd64.deb` | Debian, Ubuntu and their derivatives |
| `ferrous-lineup-…-windows-x86_64-setup.exe` | Windows on a PC, with an uninstaller and file associations |
| `ferrous-lineup-…-windows-x86_64.zip` | Windows on a PC, portable: unzip and run |
| `ferrous-lineup-…-macos-arm64.dmg` | Macs with Apple Silicon (M1 and later) |
| `ferrous-lineup-…-macos-x86_64.dmg` | Macs with an Intel processor |

The `.tar.gz` and `.zip` hold a folder with the program, the README and
the licence; a `.dmg` holds `Ferrous Lineup.app`. The AppImage is the
program in one file, and the `.deb` and Setup `.exe` install it. The `…`
is the version for a release (`v0.2.0`), or the first seven characters
of the commit for any other build (`1a2b3c4`), so you always know what
you downloaded.

## Where the downloads are

- **Every pipeline** runs the workflow: each pull request, each push to
  main, and each manual run (**Run workflow** on the workflow's page
  builds any branch on demand). Open the repository's **Actions** tab,
  choose **Release**, then the run, or click **Details** next to a
  pull request's "Build" checks. At the bottom of the run's page, under
  **Artifacts**, are the seven packages, each one a download of its own.
  Artifacts need a GitHub login to download, and GitHub deletes them
  after 90 days.
- **A tag** that starts with `v` also publishes the packages on the
  repository's **Releases** page, with a `SHA256SUMS.txt` beside
  them. A release stays there for good, and anyone can download it,
  logged in or not.

## Cutting a release

A release is a tag: a name stuck on one commit. To make version 0.2.0:

1. **Change the version** in the root `Cargo.toml`, under
   `[workspace.package]`:

   ```toml
   [workspace.package]
   version = "0.2.0"
   ```

   Every crate takes its version from there (`version.workspace =
   true`).
2. **Build once** (`cargo build`). `Cargo.lock` lists the version of
   every crate, ours too, so it changes with them. Commit both files and
   merge the change into main.
3. **Tag the commit on main and push the tag:**

   ```sh
   git checkout main
   git pull
   git tag v0.2.0
   git push origin v0.2.0
   ```

The push starts the workflow. When the four build jobs have passed, the
release appears on the Releases page, called "Ferrous Lineup v0.2.0",
with notes that list the pull requests merged since the last release.

You can also make the release on GitHub's website: **Releases > Draft
a new release**, type a new tag such as `v0.2.0`, write your own notes,
and publish. Publishing creates the tag, which starts the same
workflow, and the workflow adds the packages to the release you wrote.

Two details:

- A tag with a dash, such as `v0.2.0-beta.1`, makes a **pre-release**,
  shown as such and never as the latest release. That is how
  [semantic versioning](https://semver.org) writes a version that is not
  final yet, and Cargo reads versions the same way.
- If the tag and `Cargo.toml` disagree (a tag `v0.2.0` on a commit
  whose version is still 0.1.0), the release is still made, with a
  warning on the run's page. The tag decides the file names, so the
  packages are right, but step 1 was forgotten.

If a tag points at the wrong commit, delete the release on GitHub and
the tag (`git push origin :refs/tags/v0.2.0`, then `git tag -d
v0.2.0`), and tag again. Re-running the workflow for the same tag
replaces the release's files.

## Opening what you downloaded

The Linux and Windows programs are not **signed**, and the macOS app is
**ad-hoc signed** but not **notarized**. An ad-hoc signature is free and
is what Apple Silicon requires; notarization is the extra check Apple
does for a paid Developer ID, so Gatekeeper still asks the first time.
Windows SmartScreen asks about the unsigned Setup program too. Both
systems let you run it.

- **Linux, the AppImage**: a downloaded file is never executable, so
  allow it once and start it:

  ```sh
  chmod +x ferrous-lineup-v0.2.0-linux-x86_64.AppImage
  ./ferrous-lineup-v0.2.0-linux-x86_64.AppImage
  ```

  (In a file manager: **Properties > Permissions > Allow executing
  file as program**, then double-click it.) An AppImage mounts itself
  with FUSE, which desktop systems have; where FUSE is missing, as in
  some containers, add `--appimage-extract-and-run`.
- **Linux, the `.deb`**: install it with `apt`, which also installs the
  libraries it needs:

  ```sh
  sudo apt install ./ferrous-lineup_0.2.0-1_amd64.deb
  ```

  It puts `ferrous-lineup` in `/usr/bin`, adds the menu entry and icon,
  and installs the `.lineup` project type so file managers can open a
  project with it. Remove it with `sudo apt remove ferrous-lineup`; your
  project files stay.
- **Linux, the `.tar.gz`**: unpack and start it:

  ```sh
  tar -xzf ferrous-lineup-v0.2.0-linux-x86_64.tar.gz
  ./ferrous-lineup-v0.2.0-linux-x86_64/ferrous-lineup
  ```

  All three Linux downloads need ALSA's library (`libasound2`, on every
  desktop Linux) and load the window and graphics libraries the desktop
  already has. The plain folder also needs the keyboard library
  `libxkbcommon-x11-0`, which the AppImage brings along; the `.deb`
  declares it and apt installs it.
- **Windows, the Setup `.exe`**: run it. It installs the program, a
  Start-menu entry, an uninstaller under **Settings > Apps**, and the
  `.lineup` (and old `.loom`) file association, so double-clicking a
  project opens it in Ferrous Lineup. If a blue window says "Windows
  protected your PC", choose **More info**, then **Run anyway**.
- **Windows, the `.zip`**: unzip and double-click
  `ferrous-lineup.exe`. It is the same program without an installer or
  a file association.
- **macOS**: open the `.dmg`, then drag **Ferrous Lineup** onto the
  **Applications** folder in the same window. Start it from
  Applications. If macOS says it can't check the program for malicious
  software, try to open it once, then allow it in **System Settings >
  Privacy & Security** (on older versions, right-click the app and
  choose **Open**). You can also take away the mark macOS puts on
  downloaded files:

  ```sh
  xattr -dr com.apple.quarantine "/Applications/Ferrous Lineup.app"
  ```

To check that a download arrived whole, compare it with the checksum
the release lists. On Linux, in the folder with the downloads and
`SHA256SUMS.txt`:

```sh
sha256sum --check --ignore-missing SHA256SUMS.txt
```

On a Mac, `shasum -a 256 <file>` prints the checksum of one file, and in
Windows PowerShell `Get-FileHash <file>` does; compare it with the line
for that file in `SHA256SUMS.txt`. A **checksum** is a 64-character fingerprint of the file's bytes: change
one byte and the fingerprint is completely different.

## How it works

### Targets: which computer a program is for

A compiled program is machine code for one kind of processor, in the
file format of one operating system. Rust names each combination with a
**target triple**, written architecture-vendor-system(-environment):

| Target | Processor | System |
|--------|-----------|--------|
| `x86_64-unknown-linux-gnu` | 64-bit Intel/AMD | Linux, with the GNU C library |
| `x86_64-pc-windows-msvc` | 64-bit Intel/AMD | Windows, with Microsoft's tools |
| `aarch64-apple-darwin` | 64-bit ARM (Apple Silicon) | macOS ("Darwin" is its core) |
| `x86_64-apple-darwin` | 64-bit Intel | macOS |

`rustc --version --verbose` shows your own computer's target on the
`host:` line. `cargo build` builds for it; `cargo build --target <triple>`
builds for another one, once `rustup target add <triple>` has fetched
the standard library compiled for that target. With `--target`, the
program lands in `target/<triple>/release/` instead of
`target/release/`, so builds for different targets never mix.

In the workflow, each target is a line of a **matrix**, the same idea
as in CI: GitHub starts one job per line, on the machine the line
names, and `${{ matrix.target }}` is that line's target:

```yaml
matrix:
  include:
    - name: Linux x86_64
      os: ubuntu-22.04
      target: x86_64-unknown-linux-gnu
      platform: linux-x86_64
    # … Windows, macOS Apple Silicon, macOS Intel
```

The toolchain step installs the target (`targets: ${{ matrix.target }}`),
and the build is

```sh
cargo build --release --locked --target ${{ matrix.target }}
```

`--locked` makes Cargo build exactly the library versions written in
`Cargo.lock`, and stop with an error if the file would have to change.
A release should be built from what was tested, not from whatever
versions are newest on the day of the tag. That is also why a version
bump must commit `Cargo.lock`: with the old file, `--locked` refuses to
build.

### The Intel Mac build, made on an Apple Silicon Mac

GitHub's macOS machines are Apple Silicon. Building for another
processor than the one you build on is called **cross-compiling**, and
on a Mac it is easy: Apple's compilers and its system libraries cover
both processors, because Macs moved from Intel to Apple Silicon only a
few years ago. So the Intel job runs on the same kind of machine as the
Apple Silicon one; only its target differs. The bits of C code in the
build, such as the cryptography in `ring` (for downloading the
SoundFont over HTTPS), are compiled by the `cc` crate, which passes the
target on to Apple's C compiler.

The workflow can't run the Intel program (there is no Intel Mac to run
it on), which is fine: the tests run in CI, and the code is the same.

### Why Linux builds on an older Ubuntu

A Linux program calls the C library, glibc, for files, memory and
threads. Each glibc function carries a version, and a program records
the newest version it used: built on Ubuntu 24.04, Ferrous Lineup asks
for `GLIBC_2.39`:

```sh
$ objdump -T target/release/ferrous-lineup | grep -o 'GLIBC_[0-9.]*' | sort -Vu | tail -1
GLIBC_2.39
```

An older glibc refuses to start it ("version `GLIBC_2.39' not found"),
while a newer one runs it happily. So the Linux build runs on the oldest
Ubuntu GitHub still offers, 22.04, with glibc 2.35, and the package
runs on Ubuntu 22.04, Debian 12 and anything newer. When GitHub retires
22.04, the line moves to 24.04.

`ldd` lists the libraries a program loads when it starts:

```sh
$ ldd target/release/ferrous-lineup
	libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6
```

Only ALSA is there. The window libraries (Wayland or X11, xkbcommon
for the keyboard, OpenGL) are not missing: the crates open them while
the program runs (with `dlopen`), and choose whichever the desktop has.
That is why one Linux build works on both Wayland and X11 desktops.

### The C runtime inside the Windows program

Rust programs on Windows use Microsoft's C runtime for a few basics.
By default it is a separate DLL, `vcruntime140.dll`, which comes with
the "Visual C++ Redistributable" that most, but not all, Windows
computers have. The Windows line of the matrix adds a compiler flag:

```yaml
rustflags: -C target-feature=+crt-static
```

`crt-static` copies the runtime into the `.exe` itself (it grows a
little), so the program starts on a Windows that never
installed anything. The `RUSTFLAGS` environment variable passes flags to
every `rustc` that Cargo runs; on the other lines, `matrix.rustflags`
is empty. No console window opens behind the program, thanks to the
`windows_subsystem` line in `main.rs` from step 4
([No console window on Windows](../step-4/platforms.md#no-console-window-on-windows-cfg_attr)).

### Packing

The **Package** step runs for Linux and Windows (`if: runner.os !=
'macOS'`; Windows runs Bash too, from Git for Windows, with `shell:
bash`). It picks the label, copies the program, the README and the
licence into a folder, and packs it:

```bash
if [[ "$GITHUB_REF_TYPE" == tag ]]; then
  label="$GITHUB_REF_NAME"         # v0.2.0
else
  label="${COMMIT::7}"             # 1a2b3c4
fi
name="ferrous-lineup-$label-${{ matrix.platform }}"
```

`${COMMIT::7}` is Bash for "the first seven characters": the short
commit id, as `git log --oneline` shows it. `COMMIT` is
`${{ github.event.pull_request.head.sha || github.sha }}`: for a pull
request, GitHub builds a merge commit of its own (the branch merged
into main, to test what main would become), which you never see, so
the name takes the branch's latest commit instead. `||` picks the first
value that isn't empty.

Windows gets a `.zip`, which Windows opens by itself. Linux gets a
`.tar.gz`, because a tar file keeps file permissions: the program's
"executable" bit survives, and it starts without a `chmod +x`.

macOS does not use that step. **Package for macOS** builds a real
`Ferrous Lineup.app`: the program in `Contents/MacOS`, `Info.plist`
and the icon in `Contents/Resources`, the README and licence beside
them, and the version from `Cargo.toml` written into the plist. It
signs the app ad-hoc (`codesign --force --sign -`), puts it in a folder
with a link to `/Applications`, and `hdiutil` makes the `.dmg`. In the
mounted image, dragging the app onto that link copies it into
Applications. `COPYFILE_DISABLE=1` stops the archive tools from adding
hidden `._` files with Finder information.

The AppImage and the `.deb` are built by steps of their own, below.
The package is uploaded with `archive: false`:

```yaml
- uses: actions/upload-artifact@v7
  with:
    path: ${{ env.PACKAGE }}
    archive: false
```

By default an artifact is zipped on upload, so a `.tar.gz` would
download as a zip holding a `.tar.gz`, and the zip would lose the
executable bit anyway. With `archive: false` the file is stored as it
is, and the artifact is named after the file.

### The AppImage

An **AppImage** is a program and everything it needs, packed into one
file that starts like a program. The file is two things glued
together: a small starter program, the **runtime**, followed by a
compressed disk image (SquashFS) of a folder, the **AppDir**. When you
start the AppImage, the runtime mounts the image as a folder (with
FUSE, which lets a normal program provide a file system) and starts
the program inside. Nothing is installed, so deleting the file
uninstalls it.

The AppDir is laid out like a system's `/usr`:

```text
AppDir/
├── AppRun -> usr/bin/ferrous-lineup          what the runtime starts
├── io.github.naelolaiz.FerrousLineup.desktop -> usr/share/…  the menu entry
├── io.github.naelolaiz.FerrousLineup.svg -> usr/share/…      the icon
└── usr/
    ├── bin/ferrous-lineup
    ├── lib/libxkbcommon.so.0, libxkbcommon-x11.so.0, libxcb-xkb.so.1
    └── share/applications/…, share/icons/hicolor/scalable/apps/…
```

The **desktop entry**
([`packaging/linux/io.github.naelolaiz.FerrousLineup.desktop`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/linux/io.github.naelolaiz.FerrousLineup.desktop))
is the small text file every Linux menu reads: the app's name, its
icon, its category, and how to start it (`Exec=ferrous-lineup %f`,
where `%f` is a file to open). The icon
([`packaging/icon/ferrous-lineup.svg`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/icon/ferrous-lineup.svg))
is an F and a trumpeter, in the app's orange. Tools that add AppImages
to the menu (AppImageLauncher, for one) use both, and so does the
`.deb`.

`appimagetool` turns the AppDir into the AppImage. It and the runtime
are downloaded from their GitHub releases, pinned to one version, and
checked against the SHA-256 written in the workflow:

```bash
echo "$RUNTIME_SHA256  runtime" | sha256sum --check
```

If someone replaced the file on GitHub, the checksum would differ and
the build would stop, rather than ship whatever the file had become.

#### Bringing a library along

The AppImage carries three libraries of its own. On a system without
`libxkbcommon-x11`, the program stops before it opens a window:

```text
Library libxkbcommon-x11.so could not be loaded.
```

`ldd` didn't list it, because winit doesn't link the keyboard library
when the program is built: it opens it with **`dlopen`** when the
window is made, so one program works with X11 or Wayland, whichever is
there. A library opened this way is searched for like any other, in
this order (simplified): the folders in `LD_LIBRARY_PATH`, the
**RUNPATH** written in the program, then the system's folders. So the
workflow copies the libraries into `AppDir/usr/lib` and writes a
RUNPATH into the program with `patchelf`:

```bash
patchelf --set-rpath '$ORIGIN/../lib' AppDir/usr/bin/ferrous-lineup
patchelf --set-rpath '$ORIGIN' AppDir/usr/lib/libxkbcommon-x11.so.0
```

`$ORIGIN` means "the folder this file is in", worked out when the
program starts, so the path holds wherever the AppImage is mounted.
The second line lets `libxkbcommon-x11` find its own companions,
`libxkbcommon` and `libxcb-xkb`, next to it. The single quotes keep the
shell from replacing `$ORIGIN` with an (empty) variable.

What the AppImage does **not** bring: the C library (glibc), ALSA, and
the graphics drivers (OpenGL, Vulkan). Those must match the computer:
a graphics driver belongs to its graphics card and kernel, and ALSA to
the system's sound setup. The AppImage community keeps a list of such
libraries, the *excludelist*, that an AppImage should never carry.

### The Debian package

The `.deb` is built by `cargo-deb`, pinned to 3.8.0, from the same
release binary and the same desktop entry, icon, MIME type and AppStream
file:

```bash
cargo deb -p lineup-app --no-build --target x86_64-unknown-linux-gnu
```

`--no-build` uses the binary the matrix just built. The
`[package.metadata.deb]` table in `crates/lineup-app/Cargo.toml` lists
the files and the dependencies the package needs: ALSA, Wayland,
xkbcommon, the desktop portal, `desktop-file-utils`, `hicolor-icon-theme`
and `shared-mime-info`. That is why `sudo apt install
./ferrous-lineup_0.2.0-1_amd64.deb` installs those too, while `dpkg -i`
on its own would leave them missing.

### Publishing: a job that waits for the others

The second job, `release`, runs only for tags, and only once all four
build jobs have passed:

```yaml
release:
  if: github.ref_type == 'tag'
  needs: build
  permissions:
    contents: write
```

`needs: build` waits for every line of the matrix. If one build fails,
there is no release at all, rather than a release missing Windows.

`permissions` is about the **token** each run gets, `GITHUB_TOKEN`, a
password that expires when the job ends. At the top of the file,
`permissions: contents: read` lets every job read the code and nothing
else, so a build step can't change the repository. Only this job
asks for `contents: write`, which a release needs.

The job downloads the seven packages (each into a folder of its own
name; `skip-decompress` keeps the Windows `.zip` from being unpacked),
writes their checksums with `sha256sum`, and hands everything to `gh`,
GitHub's command-line tool, which every GitHub machine has:

```bash
if gh release view "$TAG" > /dev/null 2>&1; then
  gh release upload "$TAG" release/* --clobber
else
  gh release create "$TAG" release/* --title "Ferrous Lineup $TAG" \
    --generate-notes --verify-tag "${flags[@]}"
fi
```

- `gh release view` succeeds when the release exists already, made on
  the website or by an earlier run. Then the files are uploaded to it,
  and `--clobber` replaces files of the same name.
- `--generate-notes` writes the notes from the pull requests merged
  since the previous release.
- `--verify-tag` stops if the tag isn't on GitHub, instead of creating
  a new tag on the newest commit, so a release is always made on the
  commit you tagged.
- `flags` is a Bash **array**: `--prerelease` when the tag has a dash,
  nothing otherwise. `"${flags[@]}"` expands to its items, each as one
  argument, or to nothing at all when it is empty.

### Testing a workflow before it is merged

A release workflow only does its real work on a tag, which is a bad
moment to find a typo. Two things catch mistakes earlier:

- Every pull request runs it, so the pull request that added it built
  all the packages before it was merged, and a change that breaks the
  Windows or the Mac build shows up on its pull request. Only the
  `release` job is skipped, since a pull request is not a tag.
- [actionlint](https://github.com/rhysd/actionlint) checks workflow
  files, and, with [ShellCheck](https://www.shellcheck.net) installed,
  the shell scripts inside them:

  ```sh
  pip install actionlint-py shellcheck-py
  actionlint .github/workflows/release.yml
  ```

Finally, `concurrency` keeps one run per branch: when a pull request
gets a new push, or two pull requests are merged into main in a row,
the older run is cancelled, since the newer one contains it. Tags are
never cancelled.

## Exercises

1. **Linux on ARM.** GitHub has ARM Linux machines (`ubuntu-24.04-arm`).
   Add a matrix line for `aarch64-unknown-linux-gnu`. Which file name
   would it get, and what does it mean for the oldest glibc it runs on?
2. **One Mac program for both.** `lipo -create a b -output c` joins an
   Apple Silicon and an Intel program into one "universal" program.
   Where in the workflow could the two Mac builds meet to do that, since
   they run on different machines?
3. **Notarize the Mac app.** The workflow signs the app ad-hoc with
   `codesign --force --sign -`. With a paid Apple Developer ID, it could
   sign with `codesign --sign "Developer ID Application: …"`, submit the
   `.dmg` with `xcrun notarytool submit … --wait`, and staple Apple's
   ticket with `xcrun stapler staple`. Why can an ad-hoc signature not
   be notarized, and which of those steps needs the Apple account?
4. **Stop on a wrong version.** Make the version check fail instead of
   warn. Where should it run, so a wrong tag stops before 20 minutes of
   building, not after?
5. **Read the version in the app.** `env!("CARGO_PKG_VERSION")` is the
   crate's version, fixed at compile time. Show it in the window's
   title or in a Help > About window. Why is it `env!` and not
   `std::env::var`?

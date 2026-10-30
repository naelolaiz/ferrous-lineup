# Installing it

Ready-made programs for Linux, macOS and Windows are on the
[Releases page](https://github.com/naelolaiz/ferrous-lineup/releases)
of the project. Open the newest release and download the file for your
computer. Nothing else needs installing: the sounds, the fonts and the
video encoder come with the app.

| Download | For |
|---|---|
| `ferrous-lineup-…-linux-x86_64.AppImage` | Linux on a PC: one file, nothing to unpack |
| `ferrous-lineup_…_amd64.deb` | Debian, Ubuntu, Linux Mint and their relatives |
| `ferrous-lineup-…-linux-x86_64.tar.gz` | Linux on a PC, as a plain folder |
| `ferrous-lineup-…-windows-x86_64-setup.exe` | Windows: installs it, with an uninstaller |
| `ferrous-lineup-…-windows-x86_64.zip` | Windows, portable: unzip and run |
| `ferrous-lineup-…-macos-arm64.dmg` | Macs with Apple Silicon (M1 and later) |
| `ferrous-lineup-…-macos-x86_64.dmg` | Macs with an Intel processor |

The `…` is the version, such as `v0.4.1`. **Help > About Ferrous
Lineup** in the app shows the version you are running.

## Linux

**The AppImage.** A downloaded file is not allowed to run until you say
so. In a file manager, open the file's **Properties**, and under
**Permissions** tick **Allow executing file as program**; then
double-click it. In a terminal:

```sh
chmod +x ferrous-lineup-v0.4.1-linux-x86_64.AppImage
./ferrous-lineup-v0.4.1-linux-x86_64.AppImage
```

If it does not start and mentions FUSE, install the FUSE 2 library
(`libfuse2`, or `libfuse2t64` on newer Ubuntu), or start it with
`--appimage-extract-and-run` after its name.

**The `.deb`.** Install it with:

```sh
sudo apt install ./ferrous-lineup_0.4.0-1_amd64.deb
```

It adds Ferrous Lineup to the applications menu, and file managers open
`.lineup` projects with it. `sudo apt remove ferrous-lineup` removes it;
your songs stay where you saved them.

**The `.tar.gz`.** Unpack it and start `ferrous-lineup` in the folder:

```sh
tar -xzf ferrous-lineup-v0.4.1-linux-x86_64.tar.gz
./ferrous-lineup-v0.4.1-linux-x86_64/ferrous-lineup
```

Every desktop Linux has the sound and window libraries the app needs.
The plain folder also needs the keyboard library `libxkbcommon-x11-0`,
which the other two bring along or install.

## Windows

**The Setup program** installs Ferrous Lineup with a Start menu entry
and an uninstaller (in **Settings > Apps**), and double-clicking a
`.lineup` project opens it. The program is not signed, so the first
time Windows may show a blue window, "Windows protected your PC":
choose **More info**, then **Run anyway**.

**The `.zip`** is the same program without an installer: unzip it
anywhere and double-click `ferrous-lineup.exe`.

## macOS

Open the `.dmg` and drag **Ferrous Lineup** onto the **Applications**
folder next to it, then start it from Applications.

The app is not notarized by Apple, so the first time macOS may say it
cannot check it for malicious software. Try to open it once, then open
**System Settings > Privacy & Security** and allow it there. On older
versions of macOS, right-click the app and choose **Open** instead.

Once the app is in Applications, double-clicking a `.lineup` file in
the Finder opens it in Ferrous Lineup.

## The first start

Ferrous Lineup opens with a short song and its chord sheet, ready to
play: press **Space**. The first time, it offers to download a set of real
instrument sounds, about 30 MB, as soon as it opens; [Sounds](sounds.md)
says what that is. Then go on with [The main window](main-window.md) and [Your
first song](first-song.md).

The app starts in English. It also speaks Spanish: **Edit > Settings…
> Appearance > Language > Español** switches it at once, and it starts
in Spanish from then on.

## Checking a download

Each release lists a `SHA256SUMS.txt` file: a fingerprint of every
download. If you want to be sure a download arrived whole, put it in a
folder with that file and run, on Linux:

```sh
sha256sum --check --ignore-missing SHA256SUMS.txt
```

On a Mac, `shasum -a 256 <file>` prints a file's fingerprint, and in
Windows PowerShell `Get-FileHash <file>` does; it must match the line
for that file.

## Building it yourself

You can also build Ferrous Lineup from its source code, on any of the
three systems. The book [Inside Ferrous
Lineup](https://naelolaiz.github.io/ferrous-lineup/building.html) says
how.

# AUR

Two packages, following the AUR's usual split:

- `PKGBUILD` — the released version (0.5.0), built from the exact Git
  commit corresponding to its release tag.
- `ferrous-lineup-git/PKGBUILD` — the development version, built from
  `main`.

Both install the binary, the desktop entry, the icon, the MIME type and
the AppStream data from `packaging/`.

## Before pushing the stable package

The tag and its LFS objects must be published first. Use the packaging
recipes from `main`, not from an older release checkout. When updating
the version, set `pkgver` and pin `_commit` in `PKGBUILD` to the output
of `git rev-parse 'v<version>^{commit}'`. Git sources use `SKIP` for the
archive checksum: the commit pins the source, and Git LFS verifies each
asset's SHA-256. Then:

```sh
cd packaging/aur
makepkg --printsrcinfo > .SRCINFO
```

Then push `PKGBUILD` and `.SRCINFO` to the
`ferrous-lineup` AUR repository.

The `-git` package needs no checksum:

```sh
cd packaging/aur/ferrous-lineup-git
makepkg --printsrcinfo > .SRCINFO
```

and goes to the `ferrous-lineup-git` AUR repository.

Both packages need `git-lfs` and download the LFS images and bundled
fonts in `prepare()` before building. The stable package's `pkgrel` is
2 because its source changed from the pre-migration archive to the
rewritten Git commit. This avoids relying on GitHub's optional inclusion
of LFS objects in source archives.

After a history rewrite, publish the updated AUR recipes only after the
rewritten tags and LFS objects are available. Existing archive checksums
are no longer valid for the rewritten tags.

## Building locally

```sh
cd packaging/aur
makepkg -si
```

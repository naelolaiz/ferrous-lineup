# AUR

Two packages, following the AUR's usual split:

- `PKGBUILD` — the released version (0.4.1), built from the GitHub tag
  tarball.
- `ferrous-lineup-git/PKGBUILD` — the development version, built from
  `main`.

Both install the binary, the desktop entry, the icon, the MIME type and
the AppStream data from `packaging/`.

## Before pushing the stable package

The tag must exist first, because the checksum is of GitHub's archive of
that tag, not of the working tree. It is either pushed (`git tag v0.4.1
&& git push origin v0.4.1`) or made by the Release workflow, run by hand
on main with the tag written in its **tag** field. Then:

```sh
cd packaging/aur
updpkgsums
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

## Building locally

```sh
cd packaging/aur
makepkg -si
```

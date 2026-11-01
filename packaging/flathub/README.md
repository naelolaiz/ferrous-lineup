# Flatpak

This folder is the Flathub submission for Ferrous Lineup. Nothing here
is needed to build the program; the release workflow uses the same
desktop file, icon, MIME type and AppStream data for the AppImage and
the `.deb`.

## Files

| File | What it is |
|------|------------|
| `io.github.naelolaiz.FerrousLineup.yml` | The manifest. |
| `cargo-sources.json` | Every crate in `Cargo.lock`, by URL and SHA-256, plus a Cargo config that points at `cargo/vendor`. |
| `flathub.json` | The architectures to build. |
| `generate-sources.py` | Regenerates `cargo-sources.json`; run it after `Cargo.lock` changes. |

## Submitting

1. Tag the release (`v0.5.0`) so the manifest's `tag:` exists, and add
   the tag's commit to the manifest.
2. Fork <https://github.com/flathub/flathub>, add a branch named
   `io.github.naelolaiz.FerrousLineup`, and copy these files into it.
3. Open a pull request against `new-pr` and follow the Flathub review.

Before submitting, test the build locally:

```sh
flatpak-builder --force-clean --user --install-deps-from=flathub \
  --repo=repo build-dir packaging/flathub/io.github.naelolaiz.FerrousLineup.yml
flatpak run io.github.naelolaiz.FerrousLineup
```

## Updating

After a dependency change:

```sh
python3 packaging/flathub/generate-sources.py
```

The script only needs Python 3.11 and reads `Cargo.lock`; it does not
download anything.

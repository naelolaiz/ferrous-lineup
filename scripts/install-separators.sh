#!/usr/bin/env bash
# Install pinned, independent helper programs next to a release build.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
destination="${SEPARATOR_DIR:-$root/target/release/separators}"
mkdir -p "$destination"
destination="$(cd "$destination" && pwd)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/target/separator-build}"

checksum() {
    if command -v sha256sum >/dev/null; then
        sha256sum "$1" | cut -d ' ' -f 1
    else
        shasum -a 256 "$1" | cut -d ' ' -f 1
    fi
}

charon_model() (
    local model="$destination/htdemucs.onnx"
    local expected=68d0bf16428ef66e692cdff8a9ccf28f1ef3f69440d57e58605a4cc55fcc5e74
    if [[ -f "$model" ]]; then
        [[ "$(checksum "$model")" == "$expected" ]] || {
            echo "Unexpected model checksum: $model" >&2
            return 1
        }
        return
    fi
    local download
    download="$(mktemp "$destination/htdemucs.download.XXXXXX")"
    trap 'rm -f -- "$download"' EXIT
    curl --fail --location --output "$download" \
        https://huggingface.co/StemSplitio/htdemucs-onnx/resolve/main/htdemucs.onnx
    [[ "$(checksum "$download")" == "$expected" ]] || {
        echo "Downloaded HTDemucs model has the wrong checksum." >&2
        return 1
    }
    mv "$download" "$model"
)

if [[ $# == 0 ]]; then
    set -- charon stem-splitter demucs
fi

for backend in "$@"; do
    case "$backend" in
        charon)
            cargo install --locked --force --root "$destination" \
                --git https://github.com/Valkyra-Labs/charon-audio \
                --rev bc045cd80f009ea863772c5afef15928101df66f \
                --no-default-features --features ort-backend,decode \
                --bin charon charon-audio
            charon_model
            ;;
        stem-splitter)
            cargo install --locked --force --root "$destination" \
                --git https://github.com/gentij/stem-splitter-core \
                --rev 2631b72bc8a72afbfe3ec5d9ea926827156417a2 \
                --no-default-features --bin stem-splitter stem-splitter-core
            ;;
        demucs)
            cargo install --locked --force --root "$destination" \
                --git https://github.com/nikhilunni/demucs-rs \
                --rev 805d6f72ede20282832ac78c634e8767716c0b21 \
                --features cpu --bin demucs demucs-cli
            ;;
        *)
            echo "Unknown helper '$backend'; choose charon, stem-splitter or demucs." >&2
            exit 2
            ;;
    esac
done
echo "Helpers installed in $destination; enable matching Cargo features in the app."

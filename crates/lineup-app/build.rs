//! Embed the icon and the version in the Windows `.exe`.
//!
//! Without this, Explorer and the taskbar show a generic program, and
//! the file's Properties page has no name, version or copyright. The
//! other systems take their icons from the bundles and desktop files in
//! `packaging/`.

use std::path::Path;

fn main() {
    let icon =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging/windows/ferrous-lineup.ico");
    println!("cargo:rerun-if-changed={}", icon.display());

    // `#[cfg(windows)]` here describes the computer doing the build,
    // which is what matters: `winresource` runs the Microsoft resource
    // compiler, not the compiler for the program being built. The release
    // workflow builds Windows on Windows.
    #[cfg(windows)]
    {
        let version = env!("CARGO_PKG_VERSION");
        let mut parts = version
            .split('.')
            .map(|part| part.parse::<u64>().unwrap_or(0));
        let major = parts.next().unwrap_or(0);
        let minor = parts.next().unwrap_or(0);
        let patch = parts.next().unwrap_or(0);
        let fixed = major << 48 | minor << 32 | patch << 16;

        let mut res = winresource::WindowsResource::new();
        res.set_icon(icon.to_str().expect("the icon path is not UTF-8"));
        res.set_version_info(winresource::VersionInfo::FILEVERSION, fixed);
        res.set_version_info(winresource::VersionInfo::PRODUCTVERSION, fixed);
        res.set("CompanyName", "Ferrous Lineup contributors");
        res.set("FileDescription", "Ferrous Lineup");
        res.set("FileVersion", version);
        res.set("InternalName", "ferrous-lineup");
        res.set("LegalCopyright", "GPL-3.0-or-later");
        res.set("OriginalFilename", "ferrous-lineup.exe");
        res.set("ProductName", "Ferrous Lineup");
        res.set("ProductVersion", version);
        res.compile()
            .expect("could not embed the Windows icon and version");
    }
}

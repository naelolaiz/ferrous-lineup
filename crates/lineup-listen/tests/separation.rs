//! A real local-helper smoke test, deliberately not part of ordinary CI:
//!
//! ```text
//! LINEUP_SEPARATOR=charon LINEUP_SEPARATOR_PROGRAM=/path/to/charon \
//! LINEUP_SEPARATOR_MODEL=/path/to/htdemucs.onnx LINEUP_SEPARATOR_AUDIO=/path/to/song.wav \
//! cargo test --release -p lineup-listen --features separation-charon \
//!   --test separation -- --ignored --nocapture
//! ```
//!
//! Use `stem-splitter` or `demucs` with the corresponding feature and
//! program to test the other helpers; they do not need the model variable.

use std::path::PathBuf;

use lineup_listen::separation::{Backend, Cancellation, Config, Input, separate};

#[test]
#[ignore = "needs a compiled separator feature, local helper, model and audio; see this file"]
fn separates_with_a_real_local_helper() {
    let id = std::env::var("LINEUP_SEPARATOR").expect("set LINEUP_SEPARATOR");
    let backend = Backend::ALL
        .into_iter()
        .find(|backend| backend.id() == id)
        .expect("unknown backend");
    let config = Config {
        backend,
        program: std::env::var_os("LINEUP_SEPARATOR_PROGRAM")
            .expect("set LINEUP_SEPARATOR_PROGRAM")
            .into(),
        model: std::env::var_os("LINEUP_SEPARATOR_MODEL").map(PathBuf::from),
    };
    let path = PathBuf::from(
        std::env::var_os("LINEUP_SEPARATOR_AUDIO").expect("set LINEUP_SEPARATOR_AUDIO"),
    );
    let original = lineup_listen::decode_file(&path).unwrap();
    let stems = separate(
        Input::File {
            path: &path,
            seconds: original.seconds(),
        },
        &config,
        &Cancellation::default(),
    )
    .unwrap();
    for (name, stem) in [
        ("vocals", stems.vocals),
        ("drums", stems.drums),
        ("bass", stems.bass),
        ("other", stems.other),
    ] {
        assert!((stem.seconds() - original.seconds()).abs() <= 0.05);
        assert!(!stem.samples.is_empty() && stem.samples.iter().all(|sample| sample.is_finite()));
        println!(
            "{}: {name}, {} Hz, {:.3} s",
            backend.name(),
            stem.sample_rate,
            stem.seconds()
        );
    }
}

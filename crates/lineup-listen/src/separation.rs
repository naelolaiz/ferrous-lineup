//! Optional source separation by local helper programs. Each adapter has
//! its own Cargo feature; none is enabled by default. Separate processes
//! isolate the incompatible ONNX runtimes and backend crashes.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use lineup_core::i18n::{tr, trf};

use crate::Recording;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Charon,
    StemSplitter,
    Demucs,
}

impl Backend {
    pub const ALL: [Self; 3] = [Self::Charon, Self::StemSplitter, Self::Demucs];

    pub fn id(self) -> &'static str {
        match self {
            Self::Charon => "charon",
            Self::StemSplitter => "stem-splitter",
            Self::Demucs => "demucs",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Charon => "charon_audio",
            Self::StemSplitter => "stem-splitter-core",
            Self::Demucs => "demucs-rs",
        }
    }

    pub fn compiled(self) -> bool {
        match self {
            Self::Charon => cfg!(feature = "separation-charon"),
            Self::StemSplitter => cfg!(feature = "separation-stem-splitter"),
            Self::Demucs => cfg!(feature = "separation-demucs"),
        }
    }

    pub fn available() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter().filter(|backend| backend.compiled())
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub backend: Backend,
    /// An executable path, or a program name to find on PATH. Never a
    /// shell command: arguments are supplied by the adapter.
    pub program: PathBuf,
    /// Charon's in-graph, four-stem HTDemucs ONNX model.
    pub model: Option<PathBuf>,
}

impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        if !self.backend.compiled() {
            return Err(Error::Failed(trf(
                "The {} separator is not enabled in this build.",
                &[&self.backend.name()],
            )));
        }
        if self.program.as_os_str().is_empty() {
            return Err(Error::Failed(
                tr("Choose a separator executable.").to_string(),
            ));
        }
        if self.backend == Backend::Charon && !self.model.as_ref().is_some_and(|p| p.is_file()) {
            return Err(Error::Failed(
                tr("Choose an existing HTDemucs ONNX model for charon_audio.").to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct Cancellation(Arc<AtomicBool>);

impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn check(&self) -> Result<(), Error> {
        if self.0.load(Ordering::Relaxed) {
            Err(Error::Cancelled)
        } else {
            Ok(())
        }
    }
}

pub enum Input<'a> {
    /// The original file, not its mono analysis copy: helpers keep its
    /// stereo information and resample it themselves.
    File { path: &'a Path, seconds: f64 },
    Samples {
        samples: &'a [f32],
        sample_rate: u32,
    },
}

#[derive(Debug)]
pub struct Stems {
    pub vocals: Recording,
    pub drums: Recording,
    pub bass: Recording,
    pub other: Recording,
}

#[derive(Debug)]
pub enum Error {
    Cancelled,
    Failed(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => f.write_str(tr("Analysis was cancelled.")),
            Self::Failed(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

/// Runs the chosen helper, then reads and validates all four stems.
/// Temporary audio and logs are removed on success, failure and cancellation.
pub fn separate(input: Input<'_>, config: &Config, cancel: &Cancellation) -> Result<Stems, Error> {
    config.validate()?;
    cancel.check()?;
    #[cfg(any(
        feature = "separation-charon",
        feature = "separation-stem-splitter",
        feature = "separation-demucs"
    ))]
    {
        helper::separate(input, config, cancel)
    }
    #[cfg(not(any(
        feature = "separation-charon",
        feature = "separation-stem-splitter",
        feature = "separation-demucs"
    )))]
    {
        let _ = input;
        Err(Error::Failed(trf(
            "The {} separator is not enabled in this build.",
            &[&config.backend.name()],
        )))
    }
}

#[cfg(any(
    feature = "separation-charon",
    feature = "separation-stem-splitter",
    feature = "separation-demucs"
))]
mod helper {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    use std::process::{Child, Command, Stdio};
    use std::time::Duration;

    use super::*;

    fn io_error(error: impl fmt::Display) -> Error {
        Error::Failed(trf("Source separation failed: {}", &[&error]))
    }

    fn command(input: &Path, output: &Path, config: &Config) -> Result<Command, Error> {
        let mut command = Command::new(&config.program);
        match config.backend {
            #[cfg(feature = "separation-charon")]
            Backend::Charon => {
                let model = config.model.as_ref().ok_or_else(|| {
                    Error::Failed(
                        tr("Choose an existing HTDemucs ONNX model for charon_audio.").to_string(),
                    )
                })?;
                command
                    .arg("separate")
                    .arg(input)
                    .arg("--output")
                    .arg(output)
                    .arg("--model")
                    .arg(model.canonicalize().map_err(io_error)?)
                    .args(["--format", "wav32", "--no-server"]);
            }
            #[cfg(feature = "separation-stem-splitter")]
            Backend::StemSplitter => {
                command
                    .args(["split", "--input"])
                    .arg(input)
                    .arg("--output")
                    .arg(output)
                    .args(["--model", "htdemucs_ort_v1", "--quiet"]);
            }
            #[cfg(feature = "separation-demucs")]
            Backend::Demucs => {
                command
                    .arg(input)
                    .args(["--model", "htdemucs", "--stems", "vocals,drums,bass,other"])
                    .arg("--output")
                    .arg(output);
            }
            #[allow(unreachable_patterns)]
            _ => {
                return Err(Error::Failed(trf(
                    "The {} separator is not enabled in this build.",
                    &[&config.backend.name()],
                )));
            }
        }
        command.stdin(Stdio::null());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
        }
        Ok(command)
    }

    struct Running(Child);

    impl Drop for Running {
        fn drop(&mut self) {
            if matches!(self.0.try_wait(), Ok(Some(_))) {
                return;
            }
            if let Err(error) = self.0.kill() {
                eprintln!("Could not stop separator process {}: {error}", self.0.id());
            }
            if let Err(error) = self.0.wait() {
                eprintln!("Could not reap separator process {}: {error}", self.0.id());
            }
        }
    }

    fn log_tail(path: &Path) -> std::io::Result<String> {
        let mut file = File::open(path)?;
        let start = file.metadata()?.len().saturating_sub(8192);
        file.seek(SeekFrom::Start(start))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(String::from_utf8_lossy(&bytes).trim().to_string())
    }

    pub(super) fn separate(
        input: Input<'_>,
        config: &Config,
        cancel: &Cancellation,
    ) -> Result<Stems, Error> {
        let folder = tempfile::Builder::new()
            .prefix("ferrous-separation-")
            .tempdir()
            .map_err(io_error)?;
        let (input, seconds) = match input {
            Input::File { path, seconds } => (path.canonicalize().map_err(io_error)?, seconds),
            Input::Samples {
                samples,
                sample_rate,
            } => {
                if sample_rate == 0 || samples.iter().any(|sample| !sample.is_finite()) {
                    return Err(Error::Failed(
                        tr("Invalid audio for source separation.").to_string(),
                    ));
                }
                let path = folder.path().join("take.wav");
                crate::wav::write(samples, sample_rate, &path).map_err(io_error)?;
                (path, samples.len() as f64 / f64::from(sample_rate))
            }
        };
        if !seconds.is_finite()
            || !(0.0..=crate::decode::MAX_SECONDS).contains(&seconds)
            || seconds == 0.0
        {
            return Err(Error::Failed(
                tr("Invalid audio for source separation.").to_string(),
            ));
        }
        cancel.check()?;
        let output = folder.path().join("stems");
        std::fs::create_dir(&output).map_err(io_error)?;
        let log_path = folder.path().join("helper.log");
        let log = File::create(&log_path).map_err(io_error)?;
        let mut command = command(&input, &output, config)?;
        command
            .stdout(Stdio::from(log.try_clone().map_err(io_error)?))
            .stderr(Stdio::from(log));
        let mut running = Running(command.spawn().map_err(|error| {
            Error::Failed(trf(
                "Could not start separator {}: {}",
                &[&config.program.display(), &error],
            ))
        })?);
        let status = loop {
            cancel.check()?;
            if let Some(status) = running.0.try_wait().map_err(io_error)? {
                break status;
            }
            std::thread::sleep(Duration::from_millis(100));
        };
        if !status.success() {
            let log = log_tail(&log_path).map_err(io_error)?;
            return Err(Error::Failed(trf(
                "{} separator failed ({}): {}",
                &[&config.backend.name(), &status, &log],
            )));
        }
        cancel.check()?;
        let read = |stem: &str| {
            cancel.check()?;
            let name = if config.backend == Backend::StemSplitter {
                let prefix = input.file_stem().unwrap_or_default().to_string_lossy();
                format!("{prefix}_{stem}.wav")
            } else {
                format!("{stem}.wav")
            };
            let path = output.join(name);
            let recording = crate::decode_file(&path).map_err(|error| {
                Error::Failed(trf(
                    "Could not read separated stem {}: {}",
                    &[&stem, &error],
                ))
            })?;
            if (recording.seconds() - seconds).abs() > 0.05
                || recording.samples.iter().any(|sample| !sample.is_finite())
            {
                return Err(Error::Failed(trf(
                    "Separated stem {} has invalid audio or a different duration.",
                    &[&stem],
                )));
            }
            Ok(recording)
        };
        Ok(Stems {
            vocals: read("vocals")?,
            drums: read("drums")?,
            bass: read("bass")?,
            other: read("other")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_compiled_backends_are_available() {
        let available: Vec<_> = Backend::available().collect();
        for (backend, enabled) in [
            (Backend::Charon, cfg!(feature = "separation-charon")),
            (
                Backend::StemSplitter,
                cfg!(feature = "separation-stem-splitter"),
            ),
            (Backend::Demucs, cfg!(feature = "separation-demucs")),
        ] {
            assert_eq!(available.contains(&backend), enabled);
            if !enabled {
                let config = Config {
                    backend,
                    program: "helper".into(),
                    model: None,
                };
                assert!(
                    config
                        .validate()
                        .unwrap_err()
                        .to_string()
                        .contains(backend.name())
                );
            }
        }
    }

    #[test]
    fn cancellation_is_shared_with_the_worker() {
        let cancel = Cancellation::default();
        let worker = cancel.clone();
        assert!(worker.check().is_ok());
        cancel.cancel();
        assert!(matches!(worker.check(), Err(Error::Cancelled)));
    }

    #[cfg(all(
        unix,
        any(
            feature = "separation-charon",
            feature = "separation-stem-splitter",
            feature = "separation-demucs"
        )
    ))]
    mod helpers {
        use std::os::unix::fs::PermissionsExt;
        use std::time::{Duration, Instant};

        use super::*;

        struct Fixture {
            folder: tempfile::TempDir,
            input: PathBuf,
            config: Config,
        }

        impl Fixture {
            fn new(backend: Backend, action: &str) -> Self {
                let folder = tempfile::Builder::new()
                    .prefix("lineup helper test ")
                    .tempdir()
                    .unwrap();
                let input = folder.path().join("input with $ and ;.wav");
                let mut writer = hound::WavWriter::create(
                    &input,
                    hound::WavSpec {
                        channels: 2,
                        sample_rate: 8_000,
                        bits_per_sample: 32,
                        sample_format: hound::SampleFormat::Float,
                    },
                )
                .unwrap();
                for _ in 0..800 {
                    writer.write_sample(0.25_f32).unwrap();
                    writer.write_sample(0.75_f32).unwrap();
                }
                writer.finalize().unwrap();
                let program = folder.path().join("helper with spaces");
                let arguments = match backend {
                    Backend::Charon => "input=$2\noutput=$4",
                    Backend::StemSplitter => "input=$3\noutput=$5",
                    Backend::Demucs => "input=$1\noutput=$7",
                };
                let prefix = if backend == Backend::StemSplitter {
                    "prefix=$(basename \"$input\" .wav)_"
                } else {
                    "prefix="
                };
                std::fs::write(
                    &program,
                    format!(
                        "#!/bin/sh\nset -eu\n{arguments}\n{prefix}\n\
                     printf '%s' \"$output\" > \"$(dirname \"$0\")/output.txt\"\n{action}\n"
                    ),
                )
                .unwrap();
                std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o700)).unwrap();
                let model = folder.path().join("htdemucs model.onnx");
                std::fs::write(&model, "fixture model").unwrap();
                Self {
                    folder,
                    input,
                    config: Config {
                        backend,
                        program,
                        model: Some(model),
                    },
                }
            }

            fn input(&self) -> Input<'_> {
                Input::File {
                    path: &self.input,
                    seconds: 0.1,
                }
            }

            fn assert_workspace_removed(&self) {
                let output =
                    std::fs::read_to_string(self.folder.path().join("output.txt")).unwrap();
                assert!(!Path::new(&output).parent().unwrap().exists(), "{output}");
            }
        }

        const COPY_STEMS: &str = "for stem in vocals drums bass other; do cp \"$input\" \"$output/${prefix}${stem}.wav\"; done";

        #[test]
        fn every_compiled_helper_receives_the_original_stereo_file_and_literal_paths() {
            for backend in Backend::available() {
                let fixture = Fixture::new(backend, COPY_STEMS);
                let stems =
                    separate(fixture.input(), &fixture.config, &Cancellation::default()).unwrap();
                for stem in [stems.vocals, stems.drums, stems.bass, stems.other] {
                    assert_eq!(stem.sample_rate, 8_000);
                    assert_eq!(stem.samples.len(), 800);
                    assert!(
                        stem.samples
                            .iter()
                            .all(|sample| (*sample - 0.5).abs() < 1e-6)
                    );
                }
                assert_eq!(
                    hound::WavReader::open(&fixture.input)
                        .unwrap()
                        .spec()
                        .channels,
                    2
                );
                fixture.assert_workspace_removed();
            }
        }

        #[test]
        fn a_recorded_take_is_written_for_the_helper_without_losing_its_rate() {
            let fixture = Fixture::new(Backend::available().next().unwrap(), COPY_STEMS);
            let samples = vec![0.125; 1_600];
            let stems = separate(
                Input::Samples {
                    samples: &samples,
                    sample_rate: 16_000,
                },
                &fixture.config,
                &Cancellation::default(),
            )
            .unwrap();
            assert_eq!(stems.bass.sample_rate, 16_000);
            assert_eq!(stems.bass.samples, samples);
            fixture.assert_workspace_removed();
        }

        #[test]
        fn a_helper_failure_reports_its_exit_status_and_log() {
            let fixture = Fixture::new(
                Backend::available().next().unwrap(),
                "echo 'fixture model failure' >&2\nexit 23",
            );
            let error = separate(fixture.input(), &fixture.config, &Cancellation::default())
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("23") && error.contains("fixture model failure"),
                "{error}"
            );
            fixture.assert_workspace_removed();
        }

        #[test]
        fn missing_or_short_stems_are_errors_not_successful_empty_results() {
            let fixture = Fixture::new(Backend::available().next().unwrap(), "exit 0");
            let error = separate(fixture.input(), &fixture.config, &Cancellation::default())
                .unwrap_err()
                .to_string();
            assert!(error.contains("vocals"), "{error}");
            fixture.assert_workspace_removed();

            let fixture = Fixture::new(Backend::available().next().unwrap(), COPY_STEMS);
            let error = separate(
                Input::File {
                    path: &fixture.input,
                    seconds: 1.0,
                },
                &fixture.config,
                &Cancellation::default(),
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains("different duration"), "{error}");
            fixture.assert_workspace_removed();
        }

        #[test]
        fn an_executable_that_does_not_exist_is_reported() {
            let mut fixture = Fixture::new(Backend::available().next().unwrap(), COPY_STEMS);
            fixture.config.program = fixture.folder.path().join("missing executable");
            let error = separate(fixture.input(), &fixture.config, &Cancellation::default())
                .unwrap_err()
                .to_string();
            assert!(error.contains("Could not start separator"), "{error}");
        }

        #[test]
        fn cancelling_terminates_the_helper_and_removes_its_workspace() {
            let fixture = Fixture::new(Backend::available().next().unwrap(), "exec sleep 30");
            let cancel = Cancellation::default();
            let worker_cancel = cancel.clone();
            let path = fixture.input.clone();
            let config = fixture.config.clone();
            let worker = std::thread::spawn(move || {
                separate(
                    Input::File {
                        path: &path,
                        seconds: 0.1,
                    },
                    &config,
                    &worker_cancel,
                )
            });
            let started = Instant::now();
            let marker = fixture.folder.path().join("output.txt");
            while !marker.exists() && started.elapsed() < Duration::from_secs(5) {
                std::thread::sleep(Duration::from_millis(10));
            }
            cancel.cancel();
            assert!(matches!(worker.join().unwrap(), Err(Error::Cancelled)));
            assert!(started.elapsed() < Duration::from_secs(6));
            fixture.assert_workspace_removed();
        }
    }
}

//! Public extractors must terminate on short recordings and unusual WAV rates.

use lineup_listen::{bass, chords, drums, voice};

#[test]
fn empty_and_zero_rate_recordings_have_no_events() {
    for (samples, rate) in [(&[][..], 44_100), (&[0.0, 0.1][..], 0)] {
        assert!(voice::pitch_track(samples, rate).is_empty());
        assert!(voice::pitch_now(samples, rate).is_none());
        assert!(bass::notes(samples, rate).is_empty());
        assert!(drums::hits(samples, rate).is_empty());
        assert!(chords::heard(samples, rate).is_empty());
    }
}

#[test]
fn very_low_sample_rates_do_not_stall_the_pitch_tracker() {
    for rate in [1, 25, 49, 100, 8_000, 48_000] {
        let samples = vec![0.0; rate as usize];
        assert!(
            voice::pitch_track(&samples, rate)
                .iter()
                .all(|frame| frame.pitch.is_none())
        );
        assert!(bass::notes(&samples, rate).is_empty());
        assert!(drums::hits(&samples, rate).is_empty());
        assert!(
            chords::heard(&samples, rate)
                .iter()
                .all(|chord| chord.chord.is_none())
        );
    }
}

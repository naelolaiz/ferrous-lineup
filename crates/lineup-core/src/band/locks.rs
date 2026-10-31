//! What the band does with locked channels (see [`crate::lock`]).
//!
//! - A player that plays on a locked channel keeps its part as it is: the
//!   band does not write it again, whatever the sheet or the style says,
//!   and it stays when the band goes.
//! - The drums share the song's drum channels, so the band keeps only the
//!   notes of the locked ones and writes the others. When it has not
//!   played drums yet, the notes it keeps are the song's own, from the
//!   drum tracks it muted, so a locked kick sounds as it did.
//! - A locked "Keys" or "Bass" of the song's own is not muted when the
//!   band comes in, and the band leaves that part to it while it plays.

use super::{Muted, Player};
use crate::library::apply::{BASS, KEYS};
use crate::pattern::{NoteEvent, Pattern};
use crate::playlist::Clip;
use crate::project::{ChannelKind, Project};

impl Project {
    /// Whether `player` plays on a locked channel, so the band leaves its
    /// part as it is. The drums never do: the band keeps only the notes
    /// of their locked channels.
    pub fn plays_locked(&self, player: Player) -> bool {
        player != Player::Drums
            && self
                .band
                .as_ref()
                .and_then(|band| band.channel(player))
                .is_some_and(|channel| self.is_locked(channel))
    }

    /// Whether channel `index` is one the band mutes when it comes in: a
    /// "Keys" or "Bass" synth that plays in the song's own patterns.
    pub(super) fn accompanies(&self, index: usize) -> bool {
        self.channels().get(index).is_some_and(|channel| {
            channel.kind == ChannelKind::Synth
                && (channel.name == KEYS || channel.name == BASS)
                && self
                    .patterns
                    .iter()
                    .any(|pattern| pattern.player.is_none() && !pattern.notes(index).is_empty())
        })
    }

    /// Whether the band leaves its part to the song's own channel
    /// `index`: a locked "Keys" or "Bass" that accompanies and plays.
    pub(crate) fn band_leaves_to(&self, index: usize) -> bool {
        let Some(band) = &self.band else {
            return false;
        };
        self.is_locked(index)
            && self.accompanies(index)
            && !self.channels()[index].muted
            && !band.channels.contains(&Some(index))
    }

    /// Whether the song's own locked "Keys" plays the piano's part, or
    /// its "Bass" the bass's, so the band leaves that part out.
    pub fn song_keeps(&self, player: Player) -> bool {
        let name = match player {
            Player::Piano => KEYS,
            Player::Bass => BASS,
            _ => return false,
        };
        (0..self.channels().len())
            .any(|index| self.channels()[index].name == name && self.band_leaves_to(index))
    }

    /// Has the band take over the part of the song's own channel `index`,
    /// just unlocked: it goes quiet, as when the band came in, and plays
    /// again when the band goes.
    pub(crate) fn give_to_the_band(&mut self, index: usize) {
        let Some(name) = self
            .channels()
            .get(index)
            .map(|channel| channel.name.clone())
        else {
            return;
        };
        if let Some(band) = &mut self.band {
            band.muted.channels.push(name);
            self.channels[index].muted = true;
        }
    }

    /// The band's new drum notes, with those on locked channels replaced
    /// by what plays there now: the band's own drums, or the song's own
    /// on the tracks the band muted if the band has played none yet.
    pub(super) fn keeping_locked_drums(
        &self,
        mut notes: Vec<(usize, NoteEvent)>,
        muted: &Muted,
    ) -> Vec<(usize, NoteEvent)> {
        let locked = self.locked();
        if locked.is_empty() {
            return notes;
        }
        notes.retain(|(channel, _)| !locked.contains(*channel));
        let band_drums = self
            .patterns
            .iter()
            .any(|pattern| pattern.player == Some(Player::Drums));
        let muted_track = |clip: &Clip| {
            self.playlist
                .tracks
                .get(clip.track)
                .is_some_and(|track| muted.tracks.contains(&track.name))
        };
        let kept = self.timeline_notes(|clip, pattern| match band_drums {
            true => pattern.player == Some(Player::Drums),
            false => pattern.player.is_none() && muted_track(clip),
        });
        notes.extend(
            kept.into_iter()
                .filter(|(channel, _)| locked.contains(*channel)),
        );
        notes
    }

    /// The notes of the clips `take` picks, on the song's timeline, by
    /// channel. A clip's repeats are written out.
    fn timeline_notes(&self, take: impl Fn(&Clip, &Pattern) -> bool) -> Vec<(usize, NoteEvent)> {
        let mut notes = Vec::new();
        for clip in &self.playlist.clips {
            let pattern = &self.patterns[clip.pattern];
            if !take(clip, pattern) {
                continue;
            }
            for round in clip.rounds(pattern.length()) {
                notes.extend(
                    pattern
                        .all_notes()
                        .filter_map(|(channel, note)| Some((channel, round.place(note)?))),
                );
            }
        }
        notes
    }

    /// Before the band goes: the parts of the players on locked channels
    /// become the song's own, and so do the band's notes on locked drums
    /// that the song's own drums do not play, so nothing locked is lost.
    pub(super) fn keep_locked_parts(&mut self) {
        let Some(band) = &self.band else {
            return;
        };
        let kept: Vec<Player> = Player::ALL
            .into_iter()
            .filter(|&player| self.plays_locked(player))
            .collect();
        let songs_own = self.timeline_notes(|clip, pattern| {
            pattern.player.is_none()
                && self
                    .playlist
                    .tracks
                    .get(clip.track)
                    .is_some_and(|track| band.muted.tracks.contains(&track.name))
        });
        let drums: Vec<usize> = self
            .locked()
            .iter()
            .filter(|&channel| {
                self.patterns.iter().any(|pattern| {
                    pattern.player == Some(Player::Drums) && !pattern.notes(channel).is_empty()
                }) && !songs_own.iter().any(|(own, _)| *own == channel)
            })
            .collect();
        for pattern in &mut self.patterns {
            match pattern.player {
                Some(player) if kept.contains(&player) => pattern.player = None,
                Some(Player::Drums)
                    if drums
                        .iter()
                        .any(|&channel| !pattern.notes(channel).is_empty()) =>
                {
                    pattern.player = None;
                    for channel in 0..pattern.rows() {
                        if !drums.contains(&channel) {
                            pattern.set_notes(channel, []);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{band, part, write};
    use super::*;
    use crate::BandStyle;
    use crate::library::SONGS;
    use crate::library::demo::song_demo;
    use crate::project::{Drum, SampleSource};

    const BARS: [&str; 4] = ["C", "F", "G7", "C"];

    /// Another style in 4/4, so changing to it moves no bars.
    fn other_style(than: &str) -> &'static BandStyle {
        crate::band_styles()
            .iter()
            .find(|style| {
                style.name != than
                    && style.time == crate::TimeSignature::COMMON
                    && style.rhythm.is_some()
            })
            .unwrap()
    }

    fn drum_channel(project: &Project, drum: Drum) -> usize {
        project
            .channels()
            .iter()
            .position(|channel| channel.kind == ChannelKind::Sampler(SampleSource::Builtin(drum)))
            .unwrap()
    }

    /// Where the notes of `channel` start on the timeline, and their
    /// pitches, in the clips `take` picks.
    fn starts(
        project: &Project,
        channel: usize,
        take: impl Fn(&Clip, &Pattern) -> bool,
    ) -> Vec<(usize, u8)> {
        let mut notes: Vec<(usize, u8)> = project
            .timeline_notes(take)
            .into_iter()
            .filter(|(on, _)| *on == channel)
            .map(|(_, note)| (note.start, note.pitch.midi()))
            .collect();
        notes.sort_unstable();
        notes.dedup();
        notes
    }

    #[test]
    fn a_locked_player_keeps_its_part_and_sound() {
        let mut project = band("Pop beat", &BARS);
        let bass = project.player_channel(Player::Bass).unwrap();
        assert!(project.set_locked(bass, true));
        let (kept, sound) = (
            part(&project, Player::Bass),
            project.channels()[bass].clone(),
        );
        let piano = part(&project, Player::Piano);
        let style = other_style("Pop beat");
        project.apply_band_parts(style, None, Default::default());
        assert_eq!(part(&project, Player::Bass), kept);
        assert_eq!(project.channels()[bass], sound);
        assert_ne!(part(&project, Player::Piano), piano, "{}", style.name);
        // It keeps its notes when the chords change too, and the band
        // cannot play it again.
        write(&mut project, &["C", "Am", "G7", "C"]);
        assert_eq!(part(&project, Player::Bass), kept);
        assert!(!project.play_again(Player::Bass));
        assert!(!project.is_frozen(Player::Bass));
        // Unlocked, the band plays it from the sheet again.
        project.set_locked(bass, false);
        assert!(project.follow_chord_sheet());
        assert_ne!(part(&project, Player::Bass), kept);
        assert!(!project.is_frozen(Player::Bass));
    }

    #[test]
    fn a_locked_player_still_follows_once_a_channel_before_it_goes() {
        // A band whose guitar plays on a channel before another player's,
        // and a style without the guitar but with that player: changing
        // to it takes the guitar's channel away.
        let common = crate::TimeSignature::COMMON;
        let styles = crate::band_styles();
        let (from, to, player) = styles
            .iter()
            .filter(|style| style.time == common && style.players().contains(&Player::Guitar))
            .find_map(|from| {
                let later = [Player::Strings, Player::Organ]
                    .into_iter()
                    .filter(|player| from.players().contains(player));
                later.into_iter().find_map(|player| {
                    styles
                        .iter()
                        .find(|to| {
                            to.time == common
                                && !to.players().contains(&Player::Guitar)
                                && to.players().contains(&player)
                        })
                        .map(|to| (from, to, player))
                })
            })
            .expect("styles with a guitar and strings or an organ, and without the guitar");
        let mut project = band(from.name, &BARS);
        // Fuller, so a player that joins only in variation b plays too.
        project.set_variation(0, None, crate::band::Variation::B);
        let guitar = project.player_channel(Player::Guitar).unwrap();
        let locked = project.player_channel(player).unwrap();
        assert!(guitar < locked, "{} plays after the guitar", player.name());
        assert!(!part(&project, player).is_empty());
        assert!(project.set_locked(locked, true));
        project.set_band_style(Some(to));
        assert!(
            project.player_channel(Player::Guitar).is_none(),
            "{}",
            to.name
        );
        assert!(!project.is_frozen(player), "{} to {}", from.name, to.name);
        // Unlocked, the band plays it from the sheet again.
        let locked = project.player_channel(player).unwrap();
        assert!(project.set_locked(locked, false));
        assert!(!project.is_frozen(player));
    }

    #[test]
    fn a_locked_drum_keeps_its_notes_and_the_others_change() {
        let mut project = band("Pop beat", &BARS);
        let kick = drum_channel(&project, Drum::Kick);
        let drums = |project: &Project| part(project, Player::Drums);
        let on = |notes: Vec<(usize, usize, u8)>, channel: usize| -> Vec<(usize, usize, u8)> {
            notes.into_iter().filter(|note| note.1 == channel).collect()
        };
        let before = drums(&project);
        project.set_locked(kick, true);
        project.follow_chord_sheet();
        assert_eq!(drums(&project), before, "locking changes nothing you hear");
        let style = other_style("Pop beat");
        project.apply_band_parts(style, None, Default::default());
        let after = drums(&project);
        assert_eq!(on(after.clone(), kick), on(before.clone(), kick));
        let others = |notes: Vec<(usize, usize, u8)>| -> Vec<(usize, usize, u8)> {
            notes.into_iter().filter(|note| note.1 != kick).collect()
        };
        assert_ne!(others(after), others(before), "{}", style.name);
        assert!(!project.is_frozen(Player::Drums));
        project.set_locked(kick, false);
        assert!(project.follow_chord_sheet());
        let mut free = band("Pop beat", &BARS);
        free.apply_band_parts(style, None, Default::default());
        assert_eq!(drums(&project), drums(&free));
    }

    #[test]
    fn a_locked_keys_of_the_song_plays_in_place_of_the_band() {
        let song = SONGS.iter().find(|song| song.name == "Ode to Joy").unwrap();
        let mut project = song_demo(song);
        let keys = project
            .channels()
            .iter()
            .position(|channel| channel.name == KEYS)
            .unwrap();
        project.set_locked(keys, true);
        project.set_band_style(BandStyle::named("Bossa nova"));
        let band = project.band().unwrap();
        assert_eq!(band.muted().channels, [BASS]);
        assert!(!project.channels()[keys].muted);
        assert!(project.song_keeps(Player::Piano));
        assert!(part(&project, Player::Piano).is_empty());
        assert!(!part(&project, Player::Bass).is_empty());
        // Muted, the song's Keys hands the piano back to the band.
        project.channel_mut(keys).unwrap().muted = true;
        assert!(project.follow_chord_sheet());
        assert!(!part(&project, Player::Piano).is_empty());
        project.channel_mut(keys).unwrap().muted = false;
        assert!(project.follow_chord_sheet());
        assert!(part(&project, Player::Piano).is_empty());
        // Unlocked, the band takes the piano over, as when it came in.
        project.set_locked(keys, false);
        assert!(project.channels()[keys].muted);
        assert_eq!(project.band().unwrap().muted().channels, [BASS, KEYS]);
        assert!(project.follow_chord_sheet());
        assert!(!part(&project, Player::Piano).is_empty());
        // And it plays again when the band goes.
        project.stop_band();
        assert!(!project.channels()[keys].muted);
    }

    #[test]
    fn a_locked_drum_of_the_song_sounds_as_before_with_the_band() {
        let song = SONGS.iter().find(|song| song.name == "Ode to Joy").unwrap();
        let mut project = song_demo(song);
        let kick = drum_channel(&project, Drum::Kick);
        let songs_own = starts(&project, kick, |_, pattern| pattern.player.is_none());
        // The band mutes the song's drum track, and plays its kick.
        let drum_track = project
            .playlist()
            .tracks()
            .iter()
            .position(|track| track.name == "Drums")
            .unwrap();
        let on_the_drum_track = starts(&project, kick, |clip, pattern| {
            pattern.player.is_none() && clip.track == drum_track
        });
        assert!(!on_the_drum_track.is_empty());
        project.set_locked(kick, true);
        project.set_band_style(BandStyle::named("Bossa nova"));
        assert_eq!(project.band().unwrap().muted().tracks, ["Drums"]);
        let bands = |project: &Project| {
            starts(project, kick, |_, pattern| {
                pattern.player == Some(Player::Drums)
            })
        };
        assert_eq!(bands(&project), on_the_drum_track);
        // The other drums are the band's.
        assert!(
            part(&project, Player::Drums)
                .iter()
                .any(|&(_, channel, _)| channel != kick)
        );
        // Another style keeps it, and so does the band going away: then
        // the song's own drums play it again.
        project.apply_band_parts(other_style("Bossa nova"), None, Default::default());
        assert_eq!(bands(&project), on_the_drum_track);
        project.stop_band();
        assert_eq!(starts(&project, kick, |_, _| true), songs_own);
        assert!(project.playlist().tracks().iter().all(|track| !track.muted));
    }

    #[test]
    fn a_locked_band_bass_kept_without_the_band_does_not_double_the_songs() {
        let song = SONGS.iter().find(|song| song.name == "Ode to Joy").unwrap();
        let mut project = song_demo(song);
        project.set_band_style(BandStyle::named("Bossa nova"));
        let own = project
            .channels()
            .iter()
            .position(|channel| channel.name == "Bass")
            .unwrap();
        assert!(
            project.channels()[own].muted,
            "the band mutes the song's bass"
        );
        let band_bass = project.player_channel(Player::Bass).unwrap();
        assert_ne!(band_bass, own);
        let kept = project.channels()[band_bass].name.clone();
        project.set_locked(band_bass, true);
        project.stop_band();
        // The band's bass stays as the song's own, so the song's first
        // bass stays quiet: one bass line, not two.
        let own = project
            .channels()
            .iter()
            .position(|channel| channel.name == "Bass")
            .unwrap();
        assert!(project.channels()[own].muted);
        let kept = project
            .channels()
            .iter()
            .position(|channel| channel.name == kept)
            .unwrap();
        assert!(
            project
                .patterns()
                .iter()
                .any(|pattern| !pattern.notes(kept).is_empty())
        );
        // The song's own Keys, which no locked part replaces, plays again.
        let keys = project
            .channels()
            .iter()
            .position(|channel| channel.name == "Keys")
            .unwrap();
        assert!(!project.channels()[keys].muted);
    }

    #[test]
    fn no_band_leaves_the_locked_parts_to_the_song() {
        let mut project = band("Pop beat", &BARS);
        let bass = project.player_channel(Player::Bass).unwrap();
        let kick = drum_channel(&project, Drum::Kick);
        let name = project.channels()[bass].name.clone();
        let kept = part(&project, Player::Bass);
        let kicks = starts(&project, kick, |_, p| p.player == Some(Player::Drums));
        project.set_locked(bass, true);
        project.set_locked(kick, true);
        project.set_band_style(None);
        assert!(project.band().is_none());
        let bass = project
            .channels()
            .iter()
            .position(|channel| channel.name == name)
            .unwrap();
        assert!(project.is_locked(bass));
        let songs_own: Vec<(usize, usize, u8)> = starts(&project, bass, |_, _| true)
            .into_iter()
            .map(|(start, pitch)| (start, bass, pitch))
            .collect();
        let mut kept_starts: Vec<(usize, usize, u8)> = kept
            .into_iter()
            .map(|(start, _, pitch)| (start, bass, pitch))
            .collect();
        kept_starts.dedup();
        assert_eq!(songs_own, kept_starts);
        let kick = drum_channel(&project, Drum::Kick);
        assert_eq!(starts(&project, kick, |_, _| true), kicks);
        // The drums that were not locked went with the band.
        let others = project.patterns().iter().flat_map(|pattern| {
            pattern
                .all_notes()
                .filter(|(channel, _)| {
                    *channel != kick && !project.channels()[*channel].is_pitched()
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(others.count(), 0);
        assert!(
            project
                .patterns()
                .iter()
                .all(|pattern| pattern.player.is_none())
        );
    }

    #[test]
    fn locks_are_saved_and_a_loaded_band_plays_as_it_was() {
        let mut project = band("Pop beat", &BARS);
        let bass = project.player_channel(Player::Bass).unwrap();
        project.set_locked(bass, true);
        write(&mut project, &["C", "Am", "G7", "C"]);
        assert_eq!(crate::file::format_for(&project), 23);
        let text = crate::file::to_text(&project).unwrap();
        let mut loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert!(loaded.is_locked(bass));
        assert!(!loaded.follow_chord_sheet());
        project.set_locked(bass, false);
        assert!(crate::file::format_for(&project) < 23);
    }
}

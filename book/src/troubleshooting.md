# Troubleshooting

The app usually says what went wrong in the status bar at the bottom of
the window. Start there, then find the symptom below.

| Symptom | What to do |
|---|---|
| **No sound**, or the status bar says **Audio unavailable** | Open **Edit > Settings…**, choose **Audio**, and click the button under **Output**: **Restart audio**, or **Try again** after a failure. The app uses the system's default output device, so also check that device and its volume. |
| **A note hangs on an external MIDI synth** | Press **Space** to stop. If it still sounds, open the **MIDI** menu, choose **None** under **Output: an external synth to play**, then choose the port again. Closing the old connection sends All Notes Off on all 16 channels. |
| **"MIDI unavailable"** in the **MIDI** menu | On Linux the app needs the ALSA sequencer: the `snd-seq` kernel module and `/dev/snd/seq`. Try `sudo modprobe snd-seq`, then reopen the MIDI menu. On Windows and macOS, check that the keyboard or synth is connected and its driver is installed. |
| **"Recover unsaved work?"** at startup | The app found unsaved work from a run that did not close cleanly. **Recover it** opens that copy; **Discard it** deletes it. Save the recovered song under a new name before you close. The first change writes a copy at once, then the app writes every 30 seconds while there are unsaved changes. |
| **No recovery prompt** after a crash, though there were unsaved changes | The app may not be able to write its data folder. Recovery copies and preferences live there, by default in `~/.local/share/ferrouslineup/` on Linux, `~/Library/Application Support/Ferrous-Lineup/` on macOS, and `%APPDATA%\Ferrous Lineup\data\` on Windows. Make it writable. Saving a project somewhere else still works. |
| **"The recording is longer than 10 minutes"** | **File > Import recording…** refuses audio longer than ten minutes, because the analysis holds the recording in memory. Trim it in an audio editor and import the shorter file. |
| **A failed WAV or video export** | The app writes `song.wav.tmp` or `song.mp4.tmp` and renames it only when the export is complete, so the old file is not replaced. Free some disk space and try again. A failed export may leave its `.tmp` file behind; a cancelled one removes it. |
| **macOS will not open the app** | The app is ad-hoc signed but not notarized. Try to open it once, then allow it in **System Settings > Privacy & Security**. On older macOS versions, right-click the app and choose **Open**. |
| **Windows SmartScreen blocks the installer** | The installer is not code-signed. Choose **More info**, then **Run anyway**. |
| **The AppImage will not start (FUSE error)** | Install the FUSE 2 runtime (`libfuse2`, or `libfuse2t64` on newer Ubuntu), or run the AppImage with `--appimage-extract-and-run`. |
| **A `.lineup` file will not open by double-clicking on macOS** | The macOS bundle does not declare a `.lineup` document type, so Finder has no association for it. Open the file from **File > Open…** in the app instead. The Windows installer and the Linux package do declare the type. |
| **No Flatpak or AUR package** | The manifests are in `packaging/flathub` and `packaging/aur`, but they are not published on Flathub or the AUR yet. Use the AppImage, `.deb` or `.tar.gz` from the release, or build from source. |

## If the app itself stops

If the window closes unexpectedly, start it again: it offers the
recovery copy described above. **File > Open recent** lists the last ten
projects. If the sound card stopped but the window is still open,
**Edit > Settings… > Audio > Restart audio** starts it again without
reopening the app. [Recovery, recent projects and restarting
audio](step-9/recovery.md) has the details, and [A playback
trace](step-9/playback-trace.md#notes-that-must-not-get-lost) describes
the note-off guarantee from the code's side.

If none of this helps, the message in the status bar, the app's version
and your operating system are what an issue report needs. The project's
issues are at <https://github.com/naelolaiz/ferrous-lineup/issues>.

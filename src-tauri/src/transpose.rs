//! Transposition engine: shift chords ±N semitones with enharmonic
//! spelling chosen by the target key (flat keys prefer flats).

/// Chromatic scale, sharp spelling.
pub const SHARP_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Chromatic scale, flat spelling.
pub const FLAT_NAMES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

/// Keys whose signatures use flats (major and relative minors).
const FLAT_KEYS: [&str; 14] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "Cb", "Dm", "Gm", "Cm", "Fm", "Bbm", "Ebm", "Abm",
];

/// Convert a note name (e.g. "C#", "Bb", "E") to its pitch class 0..=11.
/// Returns None if the name is not a valid note.
pub fn note_to_pitch(note: &str) -> Option<u8> {
    let mut chars = note.chars();
    let base = match chars.next()?.to_ascii_uppercase() {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };
    let mut pitch = base as i8;
    for c in chars {
        match c {
            '#' => pitch += 1,
            'b' => pitch -= 1,
            _ => return None, // not a bare note name
        }
    }
    Some(pitch.rem_euclid(12) as u8)
}

/// Spell a pitch class with sharps or flats.
pub fn pitch_to_note(pitch: u8, use_flats: bool) -> String {
    let idx = (pitch % 12) as usize;
    if use_flats {
        FLAT_NAMES[idx].to_string()
    } else {
        SHARP_NAMES[idx].to_string()
    }
}

/// Should the given key (e.g. "Bb", "F#m") be spelled with flats?
pub fn key_prefers_flats(key: &str) -> bool {
    let key = key.trim();
    FLAT_KEYS.iter().any(|k| k.eq_ignore_ascii_case(key))
}

/// Split a chord symbol into (root, quality, optional bass note).
/// "Bbmaj7/D" -> ("Bb", "maj7", Some("D")); "Am" -> ("A", "m", None).
pub fn split_chord(chord: &str) -> Option<(String, String, Option<String>)> {
    let chord = chord.trim();
    if chord.is_empty() {
        return None;
    }
    let (head, bass) = match chord.split_once('/') {
        Some((h, b)) => (h, Some(b.to_string())),
        None => (chord, None),
    };
    let mut chars = head.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() || !matches!(first.to_ascii_uppercase(), 'A'..='G') {
        return None;
    }
    let mut root = first.to_ascii_uppercase().to_string();
    if let Some(&c) = head[1..].chars().collect::<Vec<_>>().first() {
        if c == '#' || c == 'b' {
            root.push(c);
        }
    }
    let quality = head[root.len()..].to_string();
    // Validate the bass is a bare note (allow bass quality-less notes only).
    if let Some(b) = &bass {
        if note_to_pitch(b).is_none() {
            return None;
        }
    }
    note_to_pitch(&root)?;
    Some((root, quality, bass))
}

/// Transpose a single chord symbol by `semitones` (any integer).
/// `use_flats` selects the enharmonic spelling for shifted notes.
/// Unparsable input is returned unchanged.
pub fn transpose_chord(chord: &str, semitones: i32, use_flats: bool) -> String {
    let Some((root, quality, bass)) = split_chord(chord) else {
        return chord.to_string();
    };
    let root_pitch = note_to_pitch(&root).unwrap();
    let new_root = pitch_to_note(
        ((root_pitch as i32 + semitones).rem_euclid(12)) as u8,
        use_flats,
    );
    let new_bass = bass.map(|b| {
        let p = note_to_pitch(&b).unwrap();
        pitch_to_note(((p as i32 + semitones).rem_euclid(12)) as u8, use_flats)
    });
    match new_bass {
        Some(b) => format!("{new_root}{quality}/{b}"),
        None => format!("{new_root}{quality}"),
    }
}

/// Transpose a chord in the context of a song key: the target key
/// (key + semitones) decides flat vs. sharp spelling. When the key is
/// unknown, the chord's own accidental style is preserved (flat-spelled
/// chords stay flat, everything else defaults to sharps).
pub fn transpose_chord_in_key(chord: &str, semitones: i32, key: Option<&str>) -> String {
    let use_flats = match key {
        Some(k) => {
            let target = transpose_chord(k, semitones, key_prefers_flats(k));
            key_prefers_flats(&target)
        }
        None => split_chord(chord)
            .map(|(root, _, bass)| {
                root.contains('b') || bass.as_deref().is_some_and(|b| b.contains('b'))
            })
            .unwrap_or(false),
    };
    transpose_chord(chord, semitones, use_flats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_class_roundtrip_all_12_notes() {
        for i in 0..12u8 {
            assert_eq!(note_to_pitch(SHARP_NAMES[i as usize]), Some(i));
            assert_eq!(note_to_pitch(FLAT_NAMES[i as usize]), Some(i));
        }
    }

    #[test]
    fn transpose_c_across_all_12_semitones_sharps() {
        let expected = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        for (n, want) in expected.iter().enumerate() {
            assert_eq!(
                transpose_chord("C", n as i32, false),
                *want,
                "+{n} semitones"
            );
        }
    }

    #[test]
    fn transpose_c_across_all_12_semitones_flats() {
        let expected = [
            "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
        ];
        for (n, want) in expected.iter().enumerate() {
            assert_eq!(
                transpose_chord("C", n as i32, true),
                *want,
                "+{n} semitones"
            );
        }
    }

    #[test]
    fn transpose_from_every_key_up_one_sharp_spelling() {
        // +1 from each sharp-spelled root.
        let cases = [
            ("C", "C#"),
            ("C#", "D"),
            ("D", "D#"),
            ("D#", "E"),
            ("E", "F"),
            ("F", "F#"),
            ("F#", "G"),
            ("G", "G#"),
            ("G#", "A"),
            ("A", "A#"),
            ("A#", "B"),
            ("B", "C"),
        ];
        for (from, want) in cases {
            assert_eq!(transpose_chord(from, 1, false), want, "{from} +1");
        }
    }

    #[test]
    fn enharmonic_csharp_db_equivalence() {
        assert_eq!(note_to_pitch("C#"), note_to_pitch("Db"));
        assert_eq!(transpose_chord("C", 1, false), "C#");
        assert_eq!(transpose_chord("C", 1, true), "Db");
        // Shifting either spelling by the same amount lands on the same pitch.
        assert_eq!(
            note_to_pitch(&transpose_chord("C#", 2, false)),
            note_to_pitch(&transpose_chord("Db", 2, true)),
        );
    }

    #[test]
    fn slash_chords_shift_root_and_bass() {
        assert_eq!(transpose_chord("C/G", 2, false), "D/A");
        assert_eq!(transpose_chord("G/B", -2, false), "F/A");
        assert_eq!(transpose_chord("Dm/F", 3, true), "Fm/Ab");
        assert_eq!(transpose_chord("C/E", 12, false), "C/E"); // octave: unchanged
    }

    #[test]
    fn minors_and_qualities_preserved() {
        assert_eq!(transpose_chord("Am", 3, false), "Cm");
        assert_eq!(transpose_chord("Gmaj7", 2, false), "Amaj7");
        assert_eq!(transpose_chord("Dsus4", -1, true), "Dbsus4");
        assert_eq!(transpose_chord("F#m7b5", 6, true), "Cm7b5");
        assert_eq!(transpose_chord("E7/G#", 1, true), "F7/A");
    }

    #[test]
    fn negative_and_large_intervals_wrap() {
        assert_eq!(transpose_chord("C", -1, false), "B");
        assert_eq!(transpose_chord("B", -3, true), "Ab");
        assert_eq!(transpose_chord("G", 14, false), "A"); // > octave
        assert_eq!(transpose_chord("D", -14, false), "C");
        assert_eq!(transpose_chord("E", 24, false), "E"); // two octaves
    }

    #[test]
    fn target_key_selects_enharmonic_spelling() {
        // Song in C, up a whole step -> key of D (sharp key): F# not Gb.
        assert_eq!(transpose_chord_in_key("E", 2, Some("C")), "F#");
        // Song in G, down a whole step -> key of F (flat key): Bb not A#.
        assert_eq!(transpose_chord_in_key("C", -2, Some("G")), "Bb");
        // Song in Eb, up a fourth (5 semitones) -> key of Ab (flat key): Db not C#.
        assert_eq!(transpose_chord_in_key("Ab", 5, Some("Eb")), "Db");
        // Minor keys: song in Am, up a minor third -> Cm (flat key).
        assert_eq!(transpose_chord_in_key("C", 3, Some("Am")), "Eb");
    }

    #[test]
    fn unknown_key_preserves_accidental_style() {
        assert_eq!(transpose_chord_in_key("Bb", 2, None), "C");
        assert_eq!(transpose_chord_in_key("Eb", 1, None), "E");
        assert_eq!(transpose_chord_in_key("F#", 1, None), "G");
        assert_eq!(transpose_chord_in_key("G", 1, None), "G#"); // default sharps
    }

    #[test]
    fn unparsable_chords_pass_through() {
        assert_eq!(transpose_chord("N.C.", 4, false), "N.C.");
        assert_eq!(transpose_chord("", 4, false), "");
        assert_eq!(transpose_chord("H", 4, false), "H"); // German B: not supported
    }

    #[test]
    fn key_preference_table() {
        for k in ["F", "Bb", "Eb", "Ab", "Db", "Gb", "Dm", "Gm", "Cm", "Fm"] {
            assert!(key_prefers_flats(k), "{k} should prefer flats");
        }
        for k in ["C", "G", "D", "A", "E", "B", "F#", "Em", "Bm", "F#m", "C#m"] {
            assert!(!key_prefers_flats(k), "{k} should prefer sharps");
        }
    }

    #[test]
    fn full_circle_roundtrip_all_12_keys() {
        // +12 semitones from every chromatic root returns the same note name.
        for i in 0..12u8 {
            let sharp = SHARP_NAMES[i as usize];
            let flat = FLAT_NAMES[i as usize];
            assert_eq!(transpose_chord(sharp, 12, false), sharp);
            assert_eq!(transpose_chord(flat, 12, true), flat);
            assert_eq!(transpose_chord(sharp, -12, false), sharp);
        }
    }
}

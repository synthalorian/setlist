//! ChordPro parser: directives ({title: ...}, {key: ...}, {capo: ...}) and
//! chord-over-lyric lines with inline [Chord] tokens.

use crate::transpose::transpose_chord_in_key;

/// A single parsed line of a ChordPro chart.
#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    /// {directive: value} or {directive}
    Directive { name: String, value: Option<String> },
    /// Lyric line with interleaved chords: (chord, following lyric text).
    Chords(Vec<ChordSegment>),
    /// Anything else (comments, blank lines) is preserved verbatim.
    Raw(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChordSegment {
    pub chord: Option<String>,
    pub lyric: String,
}

#[derive(Debug, Clone, Default)]
pub struct Song {
    pub lines: Vec<Line>,
}

impl Song {
    /// Look up a directive value, e.g. `song.directive("key")`.
    pub fn directive(&self, name: &str) -> Option<&str> {
        self.lines.iter().find_map(|l| match l {
            Line::Directive {
                name: n,
                value: Some(v),
            } if n.eq_ignore_ascii_case(name) => Some(v.as_str()),
            _ => None,
        })
    }
}

/// Parse ChordPro text into a Song.
pub fn parse(text: &str) -> Song {
    let lines = text.lines().map(parse_line).collect();
    Song { lines }
}

fn parse_line(line: &str) -> Line {
    let trimmed = line.trim();
    // Directive: {name: value} or {name}
    if trimmed.starts_with('{') && trimmed.ends_with('}') && trimmed.len() >= 2 {
        let inner = &trimmed[1..trimmed.len() - 1];
        // Short-form directives like {soc}/{eoc} and long {start_of_chorus}.
        let (name, value) = match inner.split_once(':') {
            Some((n, v)) => (n.trim().to_string(), Some(v.trim().to_string())),
            None => (inner.trim().to_string(), None),
        };
        if !name.is_empty() {
            return Line::Directive { name, value };
        }
    }
    // Chord line: contains at least one [Chord] token.
    if line.contains('[') {
        let mut segments = Vec::new();
        let mut rest = line;
        let mut pending_chord: Option<String> = None;
        while let Some(open) = rest.find('[') {
            let lyric_before = &rest[..open];
            if !lyric_before.is_empty() || pending_chord.is_some() {
                segments.push(ChordSegment {
                    chord: pending_chord.take(),
                    lyric: lyric_before.to_string(),
                });
            }
            rest = &rest[open + 1..];
            match rest.find(']') {
                Some(close) => {
                    pending_chord = Some(rest[..close].to_string());
                    rest = &rest[close + 1..];
                }
                None => break, // unterminated '[' — treat remainder as lyric
            }
        }
        if pending_chord.is_some() || !rest.is_empty() {
            segments.push(ChordSegment {
                chord: pending_chord,
                lyric: rest.to_string(),
            });
        }
        return Line::Chords(segments);
    }
    Line::Raw(line.to_string())
}

/// Render a Song back to ChordPro text.
pub fn render(song: &Song) -> String {
    let mut out = String::new();
    for line in &song.lines {
        match line {
            Line::Directive { name, value } => match value {
                Some(v) => {
                    out.push_str(&format!("{{{name}: {v}}}"));
                }
                None => out.push_str(&format!("{{{name}}}")),
            },
            Line::Chords(segments) => {
                for seg in segments {
                    if let Some(c) = &seg.chord {
                        out.push_str(&format!("[{c}]"));
                    }
                    out.push_str(&seg.lyric);
                }
            }
            Line::Raw(text) => out.push_str(text),
        }
        out.push('\n');
    }
    out
}

/// Transpose every chord in a ChordPro chart by `semitones`.
/// Directives are preserved; a {key: X} directive is rewritten to the new
/// key and also drives flat/sharp spelling of the transposed chords.
pub fn transpose_chart(text: &str, semitones: i32) -> String {
    let song = parse(text);
    let key = song.directive("key").map(|k| k.to_string());
    let transposed = Song {
        lines: song
            .lines
            .iter()
            .map(|line| match line {
                Line::Directive { name, value } => {
                    if name.eq_ignore_ascii_case("key") {
                        if let Some(k) = value {
                            let new_key = transpose_chord_in_key(k, semitones, Some(k.as_str()));
                            return Line::Directive {
                                name: name.clone(),
                                value: Some(new_key),
                            };
                        }
                    }
                    line.clone()
                }
                Line::Chords(segments) => Line::Chords(
                    segments
                        .iter()
                        .map(|seg| ChordSegment {
                            chord: seg
                                .chord
                                .as_ref()
                                .map(|c| transpose_chord_in_key(c, semitones, key.as_deref())),
                            lyric: seg.lyric.clone(),
                        })
                        .collect(),
                ),
                Line::Raw(_) => line.clone(),
            })
            .collect(),
    };
    render(&transposed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "{title: Amazing Grace}\n{key: G}\n{capo: 2}\n[G]Amazing [D]grace, how [Em]sweet the [C]sound\nThat [G]saved a [D]wretch like [G]me\n";

    #[test]
    fn parses_directives() {
        let song = parse(SAMPLE);
        assert_eq!(song.directive("title"), Some("Amazing Grace"));
        assert_eq!(song.directive("key"), Some("G"));
        assert_eq!(song.directive("capo"), Some("2"));
    }

    #[test]
    fn parses_chord_over_lyric_lines() {
        let song = parse(SAMPLE);
        match &song.lines[3] {
            Line::Chords(segments) => {
                let chords: Vec<_> = segments.iter().filter_map(|s| s.chord.as_deref()).collect();
                assert_eq!(chords, ["G", "D", "Em", "C"]);
                let lyrics: String = segments.iter().map(|s| s.lyric.as_str()).collect();
                assert_eq!(lyrics, "Amazing grace, how sweet the sound");
            }
            other => panic!("expected chord line, got {other:?}"),
        }
    }

    #[test]
    fn parse_render_roundtrip() {
        assert_eq!(render(&parse(SAMPLE)), SAMPLE);
    }

    #[test]
    fn transpose_rewrites_chords_and_key_directive() {
        let out = transpose_chart(SAMPLE, 2);
        assert!(out.contains("{key: A}"), "key directive updated:\n{out}");
        assert!(out.contains("[A]Amazing [E]grace"));
        assert!(out.contains("{capo: 2}"), "capo untouched");
    }

    #[test]
    fn transpose_down_uses_target_key_spelling() {
        // G down a whole step -> F, a flat key: Bb not A#.
        let out = transpose_chart(SAMPLE, -2);
        assert!(out.contains("{key: F}"));
        assert!(out.contains("[F]Amazing [C]grace"));
        assert!(out.contains("[Bb]sound"), "flat spelling:\n{out}");
    }

    #[test]
    fn transpose_preserves_lyrics_and_structure() {
        let out = transpose_chart(SAMPLE, 5);
        assert!(out.contains("Amazing "));
        assert!(out.contains("wretch like "));
        assert_eq!(out.lines().count(), SAMPLE.lines().count());
    }

    #[test]
    fn slash_chords_in_chart() {
        let text = "{key: C}\n[C]There is a [G/B]name\n";
        let out = transpose_chart(text, 2);
        assert!(out.contains("[D]There is a [A/C#]name"), "{out}");
    }
}

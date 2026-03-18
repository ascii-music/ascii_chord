use chord::Chord;
use chord::{BARRE_FRET1, BARRE_FRET2, BARRE_FRET3};
use chord::{CAPO_FRET1, CAPO_FRET2, CAPO_FRET3, CAPO_FRET4, CAPO_FRET5};
use chord::{CAPO_FRET6, CAPO_FRET7, CAPO_FRET8};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ALL_CHORDS: &'static [Chord] = &[
    // ultimate-guitar.com/lessons/chords/guide_to_chord_formation.html
    // NOTE: major is omitted such that A ≡ A major by default
    Chord::new(&["A"], "x02220", &["A"], None, None),
    Chord::new(&["A#"], "x13331", &["A♯"], None, Some(BARRE_FRET1)),
    Chord::new(&["A+"], "x03221", &["A augmented"], None, None),
    // see reddit.com/r/Guitar/comments/15a16zw/question_what_does_maj_mean
    Chord::new(&["Amaj7"], "x02120", &["A raised 7ᵗʰ"], None, None),
    Chord::new(&["A#maj7"], "x13231", &["A♯ raised 7ᵗʰ"], None, Some(BARRE_FRET1)),
    Chord::new(&["A6"], "x02222", &["A 6ᵗʰ"], None, None),
    Chord::new(&["A7"], "x02020", &["A 7ᵗʰ (shape 1)"], None, None),
    Chord::new(&["A7"], "x02223", &["A 7ᵗʰ (shape 2)"], None, None),
    Chord::new(&["A#7"], "x13131", &["A♯ 7ᵗʰ"], None, Some(BARRE_FRET1)),
    Chord::new(&["A9"], "x02423", &["A 9ᵗʰ"], None, None),
    Chord::new(&["Am"], "x02210", &["A minor"], None, None),
    Chord::new(&["A#m"], "x13321", &["A♯ minor"], None, Some(BARRE_FRET1)),
    Chord::new(&["Am7"], "x02010", &["A minor 7ᵗʰ"], None, None),
    Chord::new(&["A#m7"], "x13121", &["A♯ minor 7ᵗʰ"], None, Some(BARRE_FRET1)),
    Chord::new(&["Asus"], "xx2230", &["A suspended"], None, None),
    Chord::new(&["Asus2"], "x02200", &["A suspended 2ⁿᵈ"], None, None),
    Chord::new(&["A#sus2"], "x13311", &["A♯ suspended 2ⁿᵈ"], None, Some(BARRE_FRET1)),
    Chord::new(&["Asus4"], "x0223x", &["A suspended 4ᵗʰ"], None, None),
    Chord::new(&["A#sus4"], "x13341", &["A♯ suspended 4ᵗʰ"], None, Some(BARRE_FRET1)),
    Chord::new(&["A/C#"], "x4222x", &["A over C♯"], None, None),
    Chord::new(&["A/E"], "002220", &["A over E"], None, None),
    Chord::new(&["Am/C"], "x32210", &["A minor over C"], None, None),
    Chord::new(&["Am/G"], "302210", &["A minor over G"], None, None),
    Chord::new(&["Am/F#"], "2x2210", &["A minor over F♯"], None, None),
    Chord::new(&["B"], "xx4442", &["B"], None, None),
    Chord::new(&["B+]"], "xx3221", &["B augmented"], None, Some(BARRE_FRET3)),
    Chord::new(&["Bmaj7"], "22130x", &["B raised 7ᵗʰ"], None, None),
    Chord::new(&["B6"], "224444", &["B 6ᵗʰ"], None, None),
    Chord::new(&["B7"], "x21202", &["B 7ᵗʰ"], None, None),
    Chord::new(&["B9"], "x21222", &["B 9ᵗʰ"], None, None),
    Chord::new(&["Bm"], "x24432", &["B minor"], None, Some(BARRE_FRET2)),
    Chord::new(&["Bm"], "xx4432", &["B minor (no bar)"], None, None),
    Chord::new(&["Bm7"], "x20202", &["B minor 7ᵗʰ"], None, None),
    Chord::new(&["Bsus"], "xx3341", &["B suspended"], None, None),
    Chord::new(&["Bsus2"], "x24422", &["B suspended 2ⁿᵈ"], None, Some(BARRE_FRET2)),
    Chord::new(&["Bsus4"], "x02230", &["B suspended 4ᵗʰ"], Some(CAPO_FRET2), None),
    Chord::new(&["C"], "x32010", &["C"], None, None),
    Chord::new(&["C#"], "x02220", &["C♯"], Some(CAPO_FRET4), None),
    Chord::new(&["C+"], "x32110", &["C augmented"], None, None),
    Chord::new(&["Cmaj7"], "x32000", &["C raised 7ᵗʰ"], None, None),
    Chord::new(&["C#maj7"], "x02120", &["C♯ raised 7ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["C6"], "x02213", &["C 6ᵗʰ"], None, None),
    Chord::new(&["C7"], "x32310", &["C 7ᵗʰ"], None, None),
    Chord::new(&["C#7"], "x02020", &["C♯ 7ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["C9"], "x32333", &["C 9ᵗʰ"], None, None),
    Chord::new(&["Cm"], "x310xx", &["C minor"], None, None),
    Chord::new(&["C#m"], "x02210", &["C♯ minor"], Some(CAPO_FRET4), None),
    Chord::new(&["Cm7"], "x313xx", &["C minor 7ᵗʰ"], None, None),
    Chord::new(&["C#m7"], "x02010", &["C♯ minor 7ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["Csus"], "xx3013", &["C suspended"], None, None),
    Chord::new(&["Csus2"], "x30013", &["C suspended 2ⁿᵈ"], None, None),
    Chord::new(&["C#sus2"], "x02200", &["C♯ suspended 2ⁿᵈ"], Some(CAPO_FRET4), None),
    Chord::new(&["Csus4"], "x33011", &["C suspended 4ᵗʰ"], None, None),
    Chord::new(&["C#sus4"], "x02230", &["C♯ suspended 4ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["C/B"], "x22010", &["C over B"], None, None),
    Chord::new(&["C/E"], "032010", &["C over E"], None, None),
    Chord::new(&["C/G"], "332010", &["C over G"], None, None),
    Chord {
        short_names: &["Cadd2", "Cadd9"],
        pattern: "x32033",
        names: &["C added 2ⁿᵈ", "C added 9ᵗʰ"],
        capo: None,
        barre: None,
    },
    Chord::new(&["D"], "xx0232", &["D"], None, None),
    Chord::new(&["D#"], "x02220", &["D♯"], Some(CAPO_FRET6), None),
    Chord::new(&["D+"], "xx0332", &["D augmented"], None, None),
    Chord::new(&["Dmaj7"], "xx0222", &["D raised 7ᵗʰ"], None, None),
    Chord::new(&["D#maj7"], "xx1333", &["D♯ raised 7ᵗʰ"], None, None),
    Chord::new(&["D6"], "x00202", &["D 6ᵗʰ"], None, None),
    Chord::new(&["D7"], "xx0212", &["D 7ᵗʰ"], None, None),
    Chord::new(&["D#7"], "xx1323", &["D♯ 7ᵗʰ"], None, None),
    Chord::new(&["D9"], "200210", &["D 9ᵗʰ"], None, None),
    Chord::new(&["Dm"], "xx0231", &["D minor"], None, None),
    Chord::new(&["D#m"], "x02210", &["D♯ minor"], Some(CAPO_FRET6), None),
    Chord {
        short_names: &["Dm7", "F6"],
        pattern: "xx0211",
        names: &["D minor 7ᵗʰ", "F 6ᵗʰ"],
        capo: None,
        barre: None,
    },
    Chord::new(&["D#m7"], "xx1322", &["D♯ minor 7ᵗʰ"], None, None),
    Chord::new(&["Dsus"], "xx0233", &["D suspended"], None, None),
    Chord::new(&["Dsus2"], "xx0120", &["D suspended 2ⁿᵈ"], None, None),
    Chord::new(&["D#sus2"], "xx1341", &["D♯ suspended 2ⁿᵈ"], None, Some(BARRE_FRET1)),
    Chord::new(&["Dsus4"], "xx0233", &["D suspended 4ᵗʰ"], None, None),
    Chord::new(&["D#sus4"], "x02230", &["D♯ suspended 4ᵗʰ"], Some(CAPO_FRET6), None),
    Chord::new(&["D/A"], "x00232", &["D over A"], None, None),
    Chord::new(&["D/B"], "x20232", &["D over B"], None, None),
    Chord::new(&["D/F#"], "2x0232", &["D over F♯"], None, None),
    Chord::new(&["D/G"], "300232", &["D over G"], None, None),
    Chord::new(&["E"], "022100", &["E"], None, None),
    Chord::new(&["E+"], "03211x", &["E augmented"], None, None),
    Chord::new(&["Emaj7"], "021100", &["E raised 7ᵗʰ"], None, None),
    Chord::new(&["E6"], "022120", &["E 6ᵗʰ"], None, None),
    Chord::new(&["E7"], "020100", &["E 7ᵗʰ (shape 1)"], None, None),
    Chord::new(&["E7"], "022130", &["E 7ᵗʰ (shape 2)"], None, None),
    Chord::new(&["E9"], "020102", &["E 9ᵗʰ"], None, None),
    Chord::new(&["Em"], "022000", &["E minor"], None, None),
    Chord::new(&["Em7"], "022030", &["E minor 7ᵗʰ"], None, None),
    Chord {
        short_names: &["Esus", "Esus4"],
        pattern: "022200",
        names: &["E suspended", "E suspended 4ᵗʰ"],
        capo: None,
        barre: None,
    },
    Chord::new(&["Esus2"], "022452", &["E suspended 2ⁿᵈ"], None, None),
    Chord::new(&["E/C#"], "x42100", &["E over C♯"], None, None),
    Chord::new(&["F"], "133211", &["F"], None, Some(BARRE_FRET1)),
    Chord::new(&["F#"], "244322", &["F♯"], None, Some(BARRE_FRET2)),
    Chord::new(&["F+"], "xx3221", &["F augmented"], None, None),
    Chord::new(&["Fmaj7"], "xx3210", &["F raised 7ᵗʰ"], None, None),
    Chord::new(&["F#maj7"], "xx4321", &["F♯ raised 7ᵗʰ"], None, None),
    Chord::new(&["F7"], "131211", &["F 7ᵗʰ"], None, None),
    Chord::new(&["F#7"], "242322", &["F♯ 7ᵗʰ"], None, Some(BARRE_FRET2)),
    Chord::new(&["F9"], "xx3243", &["F 9ᵗʰ"], None, None),
    Chord::new(&["Fm"], "133111", &["F minor"], None, Some(BARRE_FRET1)),
    Chord::new(&["Fm"], "xx3111", &["F minor (no bar)"], None, None),
    Chord::new(&["F#m"], "244222", &["F♯ minor"], None, Some(BARRE_FRET2)),
    Chord::new(&["Fm7"], "131111", &["F minor 7ᵗʰ"], None, None),
    Chord::new(&["F#m7"], "242222", &["F♯ minor 7ᵗʰ"], None, Some(BARRE_FRET2)),
    Chord::new(&["Fsus"], "xx3311", &["F suspended"], None, None),
    Chord::new(&["Fsus2"], "x02200", &["F suspended 2ⁿᵈ"], Some(CAPO_FRET8), None),
    Chord::new(&["F#sus2"], "2441xx", &["F♯ suspended 2ⁿᵈ"], None, None),
    Chord::new(&["Fsus4"], "133311", &["F suspended 4ᵗʰ"], None, Some(BARRE_FRET1)),
    Chord::new(&["F#sus4"], "244422", &["F♯ suspended 4ᵗʰ"], None, Some(BARRE_FRET2)),
    Chord::new(&["F/A"], "x03211", &["F over A"], None, None),
    Chord::new(&["F/G"], "303211", &["F over G"], None, None),
    Chord::new(&["G"], "320003", &["G"], None, None),
    Chord::new(&["G#"], "022100", &["G♯"], Some(CAPO_FRET4), None),
    Chord::new(&["G+"], "321003", &["G augmented"], None, None),
    Chord::new(&["Gmaj7"], "3x0002", &["G raised 7ᵗʰ"], None, None),
    Chord::new(&["G#maj7"], "xx3210", &["G♯ raised 7ᵗʰ"], Some(CAPO_FRET3), None),
    Chord::new(&["G6"], "320000", &["G 6ᵗʰ"], None, None),
    Chord::new(&["G7"], "320001", &["G 7ᵗʰ"], None, None),
    Chord::new(&["G#7"], "020100", &["G♯ 7ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["G9"], "300201", &["G 9ᵗʰ"], None, None),
    Chord::new(&["Gm"], "xx0333", &["G minor"], None, None),
    Chord::new(&["G#m"], "022000", &["G♯ minor"], Some(CAPO_FRET4), None),
    Chord::new(&["Gm7"], "x13030", &["G minor 7ᵗʰ (shape 1)"], None, None),
    Chord::new(&["Gm7"], "020000", &["G minor 7ᵗʰ (shape 2)"], Some(CAPO_FRET3), None),
    Chord::new(&["G#m7"], "020000", &["G♯ minor 7ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["Gsus"], "xx0013", &["G suspended"], None, None),
    Chord::new(&["Gsus2"], "300033", &["G suspended 2ⁿᵈ"], None, None),
    Chord::new(&["G#sus2"], "411144", &["G♯ suspended 2ⁿᵈ"], None, Some(BARRE_FRET1)),
    Chord::new(&["Gsus4"], "330013", &["G suspended 4ᵗʰ"], None, None),
    Chord::new(&["G#sus4"], "022200", &["G♯ suspended 4ᵗʰ"], Some(CAPO_FRET4), None),
    Chord::new(&["G/B"], "x20033", &["G over B"], None, None),
    Chord::new(&["G/F"], "1x0033", &["G over F"], None, None),
    Chord::new(&["G/F#"], "220033", &["G over F♯"], None, None),
];

pub static ALL_CHORDS_BY_SHORT_NAMES: Lazy<HashMap<String, Vec<&'static Chord<'static>>>> =
    Lazy::new(|| {
        let mut map = HashMap::<_, Vec<_>>::new();

        for chord in ALL_CHORDS {
            for sn in chord.short_names {
                map.entry(sn.to_ascii_lowercase())
                    .or_default()
                    .push(chord);
            }
        }
        map
    });

#[cfg(test)]
mod tests {
    // NOTE: Useful idiom - importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_chord_pattern_length() {
        for chord in ALL_CHORDS {
            assert_eq!(
                chord.pattern.chars().count(),
                6,
                "Guitar has 6 strings. This is a invalid pattern {:?}",
                chord.pattern
            )
        }
    }

    #[test]
    fn test_digit_or_x() {
        for chord in ALL_CHORDS {
            for char in chord.pattern.chars() {
                match char.to_digit(10) {
                    None => assert_eq!(char, 'x', "Only digits or 'x' is allowed"),
                    Some(digit) => assert!(digit < 6),
                }
            }
        }
    }
}

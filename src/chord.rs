use itertools::join;

#[derive(Debug, Clone, Copy)]
pub struct Capo {
    pub fret: u8,
}

impl Capo {
    pub const fn new(fret: u8) -> Self {
        Self {
            fret: fret,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Barre {
    // Guitar normally call the string with the highest pitch, or the thinnest,
    // the 1st string; and the thickest, lowest-pitched string, the 6th string.
    // BUT, we are counting the string from left to right as 0 to 5 here.
    pub from_string: u8,
    pub to_string: u8,

    pub fret: u8,
}

impl Barre {
    pub const fn new(fret: u8) -> Self {
        Self {
            from_string: 0,
            to_string: 5,
            fret: fret,
        }
    }
}

pub const BARRE_FRET1: Barre = Barre::new(1);
pub const BARRE_FRET2: Barre = Barre::new(2);
pub const BARRE_FRET3: Barre = Barre::new(3);

pub const CAPO_FRET1: Capo = Capo::new(1);
pub const CAPO_FRET2: Capo = Capo::new(2);
pub const CAPO_FRET3: Capo = Capo::new(3);
pub const CAPO_FRET4: Capo = Capo::new(4);
pub const CAPO_FRET5: Capo = Capo::new(5);
pub const CAPO_FRET6: Capo = Capo::new(6);
pub const CAPO_FRET7: Capo = Capo::new(7);
pub const CAPO_FRET8: Capo = Capo::new(8);
// pub const CAPO_FRET9: Capo = Capo::new(9);

// monospace unicode digits, see https://www.compart.com/en/unicode/U+1D7F6
pub const MONOSP_DIGITS: [char; 10] = ['𝟶', '𝟷', '𝟸', '𝟹', '𝟺', '𝟻', '𝟼', '𝟽', '𝟾', '𝟿'];

pub fn make_fretboard(num_frets: usize) -> String {
    let mut lines = vec![
        "◯ ◯ ◯ ◯ ◯ ◯".to_string(),
        "╒═╤═╤═╤═╤═╕".to_string(),
    ];
    for i in 0..num_frets {
        lines.push("│ │ │ │ │ │".to_string());
        if i < num_frets - 1 {
            lines.push("├─┼─┼─┼─┼─┤".to_string());
        }
    }
    lines.push("└─┴─┴─┴─┴─┘".to_string());
    lines.join("\n")
}

#[derive(Debug, Clone)]
pub struct Chord<'a> {
    pub short_names: &'a [&'a str],
    // Require to be length of 6.
    // 'x' indicates that the string should be muted.
    // '0' indicates that playing the string as it (without finger placement).
    // '1' indicates that we place a finger on the first fret, '2' on the 2nd fret, and etc.
    pub pattern: &'a str,
    pub names: &'a [&'a str],
    pub capo: Option<Capo>,
    pub barre: Option<Barre>,
}

impl<'a> Chord<'a> {
    pub const fn new(
        short_names: &'a [&'a str],
        pattern: &'a str,
        names: &'a [&'a str],
        capo: Option<Capo>,
        barre: Option<Barre>,
    ) -> Self {
        Self {
            short_names: short_names,
            pattern: pattern,
            names: names,
            capo: capo,
            barre: barre,
        }
    }

    pub fn both_names(&self) -> String {
        format!(
            "{} ({})",
            join(self.names, " | "),
            join(self.short_names, " | ")
        )
    }

    pub fn max_fret(&self) -> usize {
        let pattern_max = self.pattern.chars()
            .filter_map(|c| c.to_digit(10))
            .max()
            .unwrap_or(0) as usize;
        let barre_max = self.barre.as_ref().map(|b| b.fret as usize).unwrap_or(0);
        std::cmp::max(pattern_max, barre_max)
    }

    pub fn fretboard_n(&self, num_frets: usize) -> String {
        let template = make_fretboard(num_frets);
        let mut board: Vec<char> = template.chars().collect();

        if let Some(capo) = &self.capo {
            for i in 0..5 {
                board[12 + 2 * i + 1] = MONOSP_DIGITS[capo.fret as usize]
            }
        }

        if let Some(barre) = &self.barre {
            for i in barre.from_string..barre.to_string * 2 + 1 {
                board[24 * barre.fret as usize + i as usize] = '―'
            }
        }

        for (i, ch) in self.pattern.chars().enumerate() {
            let idx: usize = i * 2;
            if ch == 'x' {
                board[idx] = '✕'
            } else {
                let value: usize = ch.to_digit(10).unwrap() as usize;
                board[idx] = ' ';
                board[idx + 24 * value] = '◯'
            }
        }

        board.iter().collect()
    }

    pub fn fretboard(&self) -> String {
        let num_frets = std::cmp::max(1, self.max_fret());
        self.fretboard_n(num_frets)
    }
}

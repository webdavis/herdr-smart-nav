#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    const ALL: [Direction; 4] = [Self::Left, Self::Down, Self::Up, Self::Right];

    pub fn parse(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|direction| direction.as_str() == word)
    }

    pub fn chord(self) -> &'static str {
        match self {
            Direction::Left => "ctrl+h",
            Direction::Down => "ctrl+j",
            Direction::Up => "ctrl+k",
            Direction::Right => "ctrl+l",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Direction::Left => "left",
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::Right => "right",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_direction_carries_its_own_chord_and_its_own_word() {
        for (word, direction, chord) in [
            ("left", Direction::Left, "ctrl+h"),
            ("down", Direction::Down, "ctrl+j"),
            ("up", Direction::Up, "ctrl+k"),
            ("right", Direction::Right, "ctrl+l"),
        ] {
            assert_eq!(Direction::parse(word), Some(direction));
            assert_eq!(direction.chord(), chord);
            assert_eq!(direction.as_str(), word, "herdr's own spelling");
        }
    }

    #[test]
    fn a_word_this_does_not_serve_is_refused() {
        assert_eq!(Direction::parse("sideways"), None);
        assert_eq!(Direction::parse(""), None);
        assert_eq!(Direction::parse("Left"), None);
    }
}

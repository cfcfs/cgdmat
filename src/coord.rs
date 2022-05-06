use anyhow::{anyhow, Context, Result};
use std::str;

#[derive(Debug)]
pub struct Coord {
    pub row: u32,
    pub col: u32,
    pub pos: u32,
}

impl Coord {
    fn get_numeric_coord(c: char) -> Result<u32> {
        c.to_digit(10)
            .map(|x| x - 1)
            .with_context(|| format!("Invalid numeric position {}", c))
    }

    fn get_alpha_coord(c: char) -> Result<u32> {
        Ok(c.to_ascii_uppercase() as u32 - ('A' as u32))
    }

    pub fn from_str(s: &str) -> Result<Coord> {
        let c: Vec<char> = s.chars().collect();
        if s.len() != 3 || !c[0].is_alphabetic() || !c[1].is_numeric() || !c[2].is_numeric() {
            Err(anyhow!("Bad coordinate format {:?}", s))
        } else {
            let t = Coord {
                row: Coord::get_alpha_coord(c[0])?,
                col: Coord::get_numeric_coord(c[1])?,
                pos: Coord::get_numeric_coord(c[2])?,
            };
            Ok(t)
        }
    }
}

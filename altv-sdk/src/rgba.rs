use crate::natives::alt_RGBA;
use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }
}

impl fmt::Debug for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rgba: [r: {}, g: {}, b: {}, a: {}]",
            self.r, self.g, self.b, self.a
        )
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rgba: [r: {}, g: {}, b: {}, a: {}]",
            self.r, self.g, self.b, self.a
        )
    }
}

impl From<alt_RGBA> for Rgba {
    fn from(c: alt_RGBA) -> Self {
        Rgba::new(c.r, c.g, c.b, c.a)
    }
}

impl From<Rgba> for alt_RGBA {
    fn from(c: Rgba) -> Self {
        alt_RGBA {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

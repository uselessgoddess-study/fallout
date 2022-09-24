use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Day(u8);

impl Day {
    #[inline]
    pub fn from_u8(n: u8) -> Option<Self> {
        (0..31).contains(&n).then(|| n).map(Self)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Month(u8);

impl Month {
    #[inline]
    pub fn from_u8(n: u8) -> Option<Self> {
        (0..12).contains(&n).then(|| n).map(Self)
    }
}

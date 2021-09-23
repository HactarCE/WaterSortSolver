use itertools::{iproduct, Itertools};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>2} -> {:<2}", self.from + 1, self.to + 1)
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle(pub Vec<Vial>);
impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n")?;
        for vial in &self.0 {
            write!(f, "  {:?}\n", vial)?;
        }
        write!(f, "]\n")?;
        Ok(())
    }
}
impl Puzzle {
    pub fn ser(&self) -> String {
        self.0.iter().map(|v| v.ser()).join(",")
    }
    pub fn deser(s: &str) -> Result<Self, &'static str> {
        let vials = s.split(',').map(|s| Vial::deser(s.trim())).try_collect()?;
        Ok(Self(vials))
    }

    pub fn new() -> Self {
        Self(vec![Vial::empty()])
    }
    pub fn push_vial(&mut self) {
        self.0.push(Vial::empty());
    }
    pub fn pop_vial(&mut self) {
        self.0.pop();
        self.last_vial();
    }
    pub fn last_vial(&mut self) -> &mut Vial {
        if self.0.is_empty() {
            self.push_vial()
        }
        self.0.last_mut().unwrap()
    }

    pub fn do_move(&self, m: Move) -> Result<Self, &'static str> {
        if m.from >= self.0.len() || m.to >= self.0.len() {
            return Err("vial index out of range");
        }
        let from = self.0[m.from];
        let to = self.0[m.to];
        let (from, to) = from.pour_into(to)?;
        let mut ret = self.clone();
        ret.0[m.from] = from;
        ret.0[m.to] = to;
        Ok(ret)
    }

    pub fn gen_all_moves<'a>(&'a self) -> impl 'a + Iterator<Item = (Move, Self)> {
        iproduct!(0..self.0.len(), 0..self.0.len())
            .map(|(from, to)| Move { from, to })
            .filter(|m| m.from != m.to) // Don't move into the same vial.
            .filter(move |m| !self.0[m.from].is_solved() && !self.0[m.to].is_solved()) // Don't mess with solved vials.
            .filter_map(move |m| Some((m, self.do_move(m).ok()?)))
    }
    pub fn is_solved(&self) -> bool {
        self.0.iter().all(|v| v.is_solved_or_empty())
    }

    pub fn vials_solved(&self) -> usize {
        self.0.iter().filter(|v| v.is_solved_or_empty()).count()
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vial(pub [Option<Color>; 4]);
impl fmt::Debug for Vial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.iter().filter_map(|&c| c).collect_vec())
    }
}
impl Vial {
    pub fn ser(self) -> String {
        self.0.iter().filter_map(|&c| c).map(Color::ser).collect()
    }
    pub fn deser(s: &str) -> Result<Self, &'static str> {
        if s.len() > 4 {
            return Err("too many colors in one vial");
        }

        let mut colors = s
            .chars()
            .map(Color::deser)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();

        Ok(Self([
            colors.next(),
            colors.next(),
            colors.next(),
            colors.next(),
        ]))
    }

    pub fn empty() -> Self {
        Self([None; 4])
    }
    pub fn top_color(&self) -> Option<Color> {
        self.0.iter().filter_map(|&c| c).last()
    }
    fn last_full_slot(&mut self) -> Result<&mut Option<Color>, &'static str> {
        self.0
            .iter_mut()
            .filter(|c| c.is_some())
            .last()
            .ok_or("vial is empty")
    }
    fn first_empty_slot(&mut self) -> Result<&mut Option<Color>, &'static str> {
        self.0
            .iter_mut()
            .filter(|c| c.is_none())
            .next()
            .ok_or("vial is full")
    }
    pub fn push(mut self, color: Color) -> Result<Self, &'static str> {
        *self.first_empty_slot()? = Some(color);
        Ok(self)
    }
    pub fn push_color(mut self, color: Color) -> Result<Self, &'static str> {
        if self.is_empty() || self.top_color() == Some(color) {
            *self.first_empty_slot()? = Some(color);
            Ok(self)
        } else {
            Err("top color does not match")
        }
    }
    pub fn pop(mut self) -> Result<Self, &'static str> {
        *self.last_full_slot()? = None;
        Ok(self)
    }
    pub fn pop_color(self, color: Color) -> Result<Self, &'static str> {
        if self.top_color() == Some(color) {
            self.pop()
        } else {
            Err("wrong top color")
        }
    }
    pub fn pour_into(mut self, mut other: Self) -> Result<(Self, Self), &'static str> {
        let c = self.last_full_slot()?.unwrap();
        self = self.pop_color(c)?;
        other = other.push_color(c)?;
        while let (Ok(s), Ok(o)) = (self.pop_color(c), other.push_color(c)) {
            self = s;
            other = o;
        }
        Ok((self, other))
    }

    pub fn is_solved(self) -> bool {
        self.is_solved_or_empty() && self.0[0].is_some()
    }
    pub fn is_empty(self) -> bool {
        self.is_solved_or_empty() && self.0[0].is_none()
    }
    pub fn is_solved_or_empty(self) -> bool {
        self.0.iter().all_equal()
    }
}

pub const COLORS: &[Color] = &[
    Color::Cornflower,
    Color::Grey,
    Color::Mint,
    Color::Navy,
    Color::Orange,
    Color::Pickle,
    Color::Pink,
    Color::Purple,
    Color::Red,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Color {
    Cornflower,
    Grey,
    Mint,
    Navy,
    Orange,
    Pickle,
    Pink,
    Purple,
    Red,
}
impl Color {
    pub fn ser(self) -> char {
        match self.into() {
            x @ 0..=25 => ('A' as u8 + x) as char,
            26.. => unimplemented!("cannot serialize >26 colors"),
        }
    }
    pub fn deser(c: char) -> Result<Self, &'static str> {
        match c {
            'A'..='Z' => Color::try_from(c as u8 - 'A' as u8).map_err(|_| "color out of range"),
            _ => Err("unknown color symbol"),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Color::Cornflower => "cornflower",
            Color::Grey => "grey",
            Color::Mint => "mint",
            Color::Navy => "navy",
            Color::Orange => "orange",
            Color::Pickle => "pickle",
            Color::Pink => "pink",
            Color::Purple => "purple",
            Color::Red => "red",
        }
    }
    pub fn simple_name(self) -> &'static str {
        match self {
            Color::Cornflower => "blue",
            Color::Grey => "grey",
            Color::Mint => "green",
            Color::Navy => "blue",
            Color::Orange => "orange",
            Color::Pickle => "green",
            Color::Pink => "pink",
            Color::Purple => "purple",
            Color::Red => "red",
        }
    }
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Cornflower => (118, 162, 226),
            Color::Grey => (99, 99, 100),
            Color::Mint => (146, 210, 126),
            Color::Navy => (61, 56, 191),
            Color::Orange => (205, 138, 69),
            Color::Pickle => (128, 147, 33),
            Color::Pink => (200, 97, 120),
            Color::Purple => (99, 52, 144),
            Color::Red => (164, 50, 37),
        }
    }
}

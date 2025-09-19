use std::fmt::Display;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Offset {
    x: i8,
    y: i8,
}

impl Offset {
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
    const fn in_range(self) -> bool {
        return self.x.abs() < 8 && self.y.abs() < 8;
    }
    pub const fn mul(self, rhs: i8) -> Option<Self> {
        let multiplied = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
        if multiplied.in_range() {
            return Some(multiplied);
        }
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(transparent)]
pub struct Position {
    pub index: u8,
}

impl Deref for Position {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl Position {
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }

        let mut output = 0;
        let mut bytes = s.bytes();
        match bytes.next().unwrap() {
            i @ b'a'..=b'h' => output += i - b'a',
            _ => return None,
        }
        match bytes.next().unwrap() {
            i @ b'1'..=b'8' => output += (i - b'1') * 8,
            _ => return None,
        }

        return Some(Position::from_index(output));
    }
    #[inline]
    pub const fn new(x: u8, y: u8) -> Self {
        assert!(x < 8 && y < 8, "Value out of range");
        Self {
            index: x | (y << 3),
        }
    }

    #[inline]
    pub const fn from_index(index: u8) -> Self {
        Self { index }
    }

    #[inline]
    pub const fn as_tuple(self) -> (u8, u8) {
        (self.x(), self.y())
    }

    #[inline]
    pub const fn x(self) -> u8 {
        self.index & 0b000111
    }

    #[inline]
    pub const fn y(self) -> u8 {
        self.index >> 3
    }

    #[inline]
    pub const fn index(self) -> u8 {
        self.index
    }

    pub fn with_offset(self, offset: Offset) -> Option<Self> {
        let offset_self;
        if offset.x < 0 {
            offset_self = self.sub_x(offset.x.abs() as u8)?;
        } else {
            offset_self = self.add_x(offset.x as u8)?;
        }
        if offset.y < 0 {
            offset_self.sub_y(offset.y.abs() as u8)
        } else {
            offset_self.add_y(offset.y as u8)
        }
    }
    pub const fn as_mask(&self) -> u64 {
        1 << self.index
    }
    #[inline]
    pub fn with_x(self, x: u8) -> Option<Self> {
        if x >= 8 {
            return None;
        }
        Some(Position::new(x, self.y()))
    }

    #[inline]
    pub fn with_y(self, y: u8) -> Option<Self> {
        if y >= 8 {
            return None;
        }
        Some(Position::new(self.x(), y))
    }

    #[inline]
    pub fn add_x(self, rhs: u8) -> Option<Self> {
        let x = self.x().checked_add(rhs)?;
        self.with_x(x)
    }

    #[inline]
    pub fn add_y(self, rhs: u8) -> Option<Self> {
        let y = self.y().checked_add(rhs)?;
        self.with_y(y)
    }

    #[inline]
    pub fn sub_x(self, rhs: u8) -> Option<Self> {
        let x = self.x().checked_sub(rhs)?;
        self.with_x(x)
    }

    #[inline]
    pub fn sub_y(self, rhs: u8) -> Option<Self> {
        let y = self.y().checked_sub(rhs)?;
        self.with_y(y)
    }

    #[inline]
    pub fn snap_to_side(self) -> Self {
        if self.x() <= 3 {
            self.with_x(0).unwrap()
        } else {
            self.with_x(7).unwrap()
        }
    }
}

impl TryFrom<u8> for Position {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 63 {
            return Err(());
        }
        Ok(Self { index: value })
    }
}

impl TryFrom<(u8, u8)> for Position {
    type Error = ();
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        if value.0 > 7 || value.1 > 7 {
            return Err(());
        }
        Ok(Self::new(value.0, value.1))
    }
}
impl Into<usize> for Position {
    fn into(self) -> usize {
        *self as usize
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(vec![self.x() + b'a', self.y() + b'1']).unwrap()
        )
    }
}

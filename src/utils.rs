use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: Position,
    to: Position,
    promote_to: Option<PieceType>,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
            promote_to: None,
        }
    }
    pub fn with_promotion(self, promote_to: PieceType) -> Self {
        Self {
            promote_to: Some(promote_to),
            ..self
        }
    }
}

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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Position {
    index: u8,
}

impl Deref for Position {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl Position {
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
        self.index & 0b111000
    }

    #[inline]
    pub const fn index(self) -> u8 {
        self.index
    }

    pub fn with_offset(self, offset: Offset) -> Option<Self> {
        let offset_self;
        if offset.x < 0 {
            offset_self = self.sub_x(offset.x.abs() as u8);
            if offset_self == None {
                return None;
            }
        } else {
            offset_self = self.add_x(offset.x as u8);
            if offset_self == None {
                return None;
            }
        }
        if offset.y < 0 {
            offset_self.unwrap().sub_y(offset.y.abs() as u8)
        } else {
            offset_self.unwrap().add_y(offset.y as u8)
        }
    }

    #[inline]
    pub const fn with_x(self, x: u8) -> Self {
        Position::new(x, self.y())
    }

    #[inline]
    pub const fn with_y(self, y: u8) -> Self {
        Position::new(self.x(), y)
    }

    #[inline]
    pub fn add_x(self, rhs: u8) -> Option<Self> {
        self.x().checked_add(rhs).map(|x| self.with_x(x))
    }

    #[inline]
    pub fn add_y(self, rhs: u8) -> Option<Self> {
        self.y().checked_add(rhs).map(|y| self.with_y(y))
    }

    #[inline]
    pub fn sub_x(self, rhs: u8) -> Option<Self> {
        self.x().checked_sub(rhs).map(|x| self.with_x(x))
    }

    #[inline]
    pub fn sub_y(self, rhs: u8) -> Option<Self> {
        self.y().checked_sub(rhs).map(|y| self.with_y(y))
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[inline]
pub fn occupied(bitboard: u64, pos: Position) -> bool {
    bitboard & (1 << pos.index) != 0
}

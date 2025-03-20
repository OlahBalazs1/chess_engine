pub struct Move{
    from: Position,
    to: Position,
    promote_to: Option<PieceType>
}

impl Move{
    pub fn new(from: Position, to: Position) -> Self{
        Self { from, to, promote_to: None }
    }
    pub fn with_promotion(self, promote_to: PieceType) -> Self{
        Self{
            promote_to: Some(promote_to),
            ..self
        }

    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Position{
    index: u8
}

impl Position{
    #[inline]
    pub fn new(x: u8, y: u8) -> Self{
        assert!(x < 8 && y < 8, "Value out of range");
        Self { index: x | (y << 3) }
    }

    #[inline]
    pub fn from_index(index: u8) -> Self{
        Self { index }
    }
    
    #[inline]
    pub fn as_tuple(self) -> (u8, u8){
        (self.x(), self.y())
    }

    #[inline]
    pub fn x(self) -> u8{
        self.index & 000111
    }

    #[inline]
    pub fn y(self) -> u8{
        self.index & 111000
    }

    pub fn index(self) -> u8{
        self.index
    }
    
    #[inline]
    pub fn with_x(self, x: u8) -> Self{
        Position::new(x, self.y())
    }
    
    #[inline]
    pub fn with_y(self, y: u8) -> Self{
        Position::new(self.x(), y)
    }
    
    #[inline]
    pub fn add_x(self, rhs: u8) -> Option<Self>{
        self.x().checked_add(rhs).map(|x| self.with_x(x))
    }
    
    #[inline]
    pub fn add_y(self, rhs: u8) -> Option<Self>{
        self.y().checked_add(rhs).map(|y| self.with_y(y))
    }
    
    #[inline]
    pub fn sub_x(self, rhs: u8) -> Option<Self>{
        self.x().checked_sub(rhs).map(|x| self.with_x(x))
    }
    
    #[inline]
    pub fn sub_y(self, rhs: u8) -> Option<Self>{
        self.y().checked_sub(rhs).map(|y| self.with_y(y))
    }
}

impl TryFrom<u8> for Position{
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error>{
        if value > 63 {
            return Err(())
        }
        Ok(Self { index: value })
    }
}

impl TryFrom<(u8, u8)> for Position{
    type Error = ();
    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error>{
        if value.0 > 7 || value.1 > 7{
            return Err(())
        }
        Ok(Self::new(value.0, value.1))
    }
}


enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}
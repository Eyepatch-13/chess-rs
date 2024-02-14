trait Movement {
    fn moves(&self);
}

#[derive(Clone, Debug)]
pub enum PieceType {
    King {
        in_check: bool,
        has_moved: bool,
        is_checkmated: bool,
    }, Queen, Bishop, Knight
    , Rook {
        has_moved: bool,
    }
    , Pawn {
        has_moved: bool,
    },
}

impl PieceType {
    pub fn new(piece: &str) -> Self {
        match piece {
            "King" => {
                return PieceType::King {
                    is_checkmated: false,
                    in_check: false,
                    has_moved: false,
                };
            },
            "Queen" => {
                return PieceType::Queen;
            },
            "Bishop" => {
                return PieceType::Bishop;
            },
            "Knight" => {
                return PieceType::Knight;
            },
            "Rook" => {
                return PieceType::Rook {
                    has_moved: false,
                };
            },
            "Pawn" => {
                return PieceType::Pawn {
                    has_moved: false
                };
            },
            _ => {
                panic!("Not a Piece Type or Incorrect Spelling \" Eg- Pawn and not pawn\"");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PieceColor {
    White, Black,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Piece {
            piece_type,
            piece_color,
        }
    }

    pub fn get_position(self) -> (usize, usize) {
        todo!();
    }
}

impl Movement for Piece {
    fn moves(&self) {
        todo!();
    }
}


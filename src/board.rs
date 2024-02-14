use crate::piece::*;

#[derive(Debug)]
pub struct Board {
    pub grid: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new() -> Self {
        let wpawn: Vec<Option<Piece>> = vec![Some(Piece::new(PieceType::new("Pawn"), PieceColor::White)); 8];
        let bpawn: Vec<Option<Piece>> = vec![Some(Piece::new(PieceType::new("Pawn"), PieceColor::Black)); 8];
        let blank: Vec<Option<Piece>> = vec![None; 8];
        let mut wpiece: Vec<Option<Piece>> = vec![None; 8];
        let mut bpiece = wpiece.clone();
        wpiece[0] = Some(Piece::new(PieceType::new("Rook"), PieceColor::White));
        wpiece[1] = Some(Piece::new(PieceType::new("Knight"), PieceColor::White));
        wpiece[2] = Some(Piece::new(PieceType::new("Bishop"), PieceColor::White));
        wpiece[3] = Some(Piece::new(PieceType::new("Queen"), PieceColor::White));
        wpiece[4] = Some(Piece::new(PieceType::new("King"), PieceColor::White));
        wpiece[5] = Some(Piece::new(PieceType::new("Bishop"), PieceColor::White));
        wpiece[6] = Some(Piece::new(PieceType::new("Knight"), PieceColor::White));
        wpiece[7] = Some(Piece::new(PieceType::new("Rook"), PieceColor::White));
        bpiece[0] = Some(Piece::new(PieceType::new("Rook"), PieceColor::Black));
        bpiece[1] = Some(Piece::new(PieceType::new("Knight"), PieceColor::Black));
        bpiece[2] = Some(Piece::new(PieceType::new("Bishop"), PieceColor::Black));
        bpiece[3] = Some(Piece::new(PieceType::new("Queen"), PieceColor::Black));
        bpiece[4] = Some(Piece::new(PieceType::new("King"), PieceColor::Black));
        bpiece[5] = Some(Piece::new(PieceType::new("Bishop"), PieceColor::Black));
        bpiece[6] = Some(Piece::new(PieceType::new("Knight"), PieceColor::Black));
        bpiece[7] = Some(Piece::new(PieceType::new("Rook"), PieceColor::Black));
        Board {
            grid: [bpiece, bpawn, blank.clone(), blank.clone(), blank.clone(), blank.clone(), wpawn, wpiece].to_vec(),
        }
    }
}


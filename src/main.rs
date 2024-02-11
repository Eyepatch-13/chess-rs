use ggez::{ContextBuilder,
    conf::WindowMode,
    GameResult,
    event::{self, EventHandler},
    graphics::{self, Color, Rect, Mesh, DrawParam, Image},
    glam::vec2};

const CELL_SIZE: (f32, f32) = (120.0, 120.0);
const GRID_SIZE: (f32, f32) = (8.0, 8.0);
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);

trait Movement {
    fn moves(&self);
}

#[derive(Clone, Debug)]
enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn,
}

#[derive(Clone, Debug)]
enum PieceColor {
    White, Black,
}

#[derive(Clone, Debug)]
struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
}

impl Piece {
    fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Piece {
            piece_type,
            piece_color,
        }
    }
}

impl Movement for Piece {
    fn moves(&self) {
        todo!();
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Option<Piece>>>,
}

impl Board {
    fn new() -> Self {
        let wpawn: Vec<Option<Piece>> = vec![Some(Piece::new(PieceType::Pawn, PieceColor::White)); 8];
        let bpawn: Vec<Option<Piece>> = vec![Some(Piece::new(PieceType::Pawn, PieceColor::Black)); 8];
        let blank: Vec<Option<Piece>> = vec![None; 8];
        let mut wpiece: Vec<Option<Piece>> = vec![None; 8];
        let mut bpiece = wpiece.clone();
        wpiece[0] = Some(Piece::new(PieceType::Rook, PieceColor::White));
        wpiece[1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        wpiece[2] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        wpiece[3] = Some(Piece::new(PieceType::Queen, PieceColor::White));
        wpiece[4] = Some(Piece::new(PieceType::King, PieceColor::White));
        wpiece[5] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        wpiece[6] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        wpiece[7] = Some(Piece::new(PieceType::Rook, PieceColor::White));
        bpiece[0] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        bpiece[1] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        bpiece[2] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        bpiece[3] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
        bpiece[4] = Some(Piece::new(PieceType::King, PieceColor::Black));
        bpiece[5] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        bpiece[6] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        bpiece[7] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        Board {
            grid: [bpiece, bpawn, blank.clone(), blank.clone(), blank.clone(), blank.clone(), wpawn, wpiece].to_vec(),
        }
    }
}

impl EventHandler for Board {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for i in 0..GRID_SIZE.0 as usize{
            for j in 0..GRID_SIZE.1 as usize{
                let color = if (i+j)%2 == 0 {Color::YELLOW} else {Color::CYAN};
                let rect = Mesh::new_rectangle(ctx,
                                               graphics::DrawMode::fill(),
                                               Rect::new(
                                                   i as f32 * CELL_SIZE.0,
                                                   j as f32 * CELL_SIZE.1,
                                                   CELL_SIZE.0,
                                                   CELL_SIZE.1,
                                                   ), color)?;
                canvas.draw(&rect, DrawParam::default());

                let path = match &self.grid[j][i] {
                    Some(piece) => {
                        match (&piece.piece_type, &piece.piece_color) {
                            (PieceType::King, PieceColor::White) => {"/white-king.png"},
                            (PieceType::Queen, PieceColor::White) => {"/white-queen.png"},
                            (PieceType::Bishop, PieceColor::White) => {"/white-bishop.png"},
                            (PieceType::Knight, PieceColor::White) => {"/white-knight.png"},
                            (PieceType::Rook, PieceColor::White) => {"/white-rook.png"},
                            (PieceType::Pawn, PieceColor::White) => {"/white-pawn.png"},
                            (PieceType::King, PieceColor::Black) => {"/black-king.png"},
                            (PieceType::Queen, PieceColor::Black) => {"/black-queen.png"},
                            (PieceType::Bishop, PieceColor::Black) => {"/black-bishop.png"},
                            (PieceType::Knight, PieceColor::Black) => {"/black-knight.png"},
                            (PieceType::Rook, PieceColor::Black) => {"/black-rook.png"},
                            (PieceType::Pawn, PieceColor::Black) => {"/black-pawn.png"},
                        }
                    },
                    None => continue,
                };

                let image = Image::from_path(ctx, path)?;
                canvas.draw(&image, DrawParam::default().dest(vec2(i as f32 * CELL_SIZE.0, j as f32 * CELL_SIZE.1)));
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let board = Board::new();
    let (ctx, event_loop) = ContextBuilder::new("chess-rs", "Eyepatch")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;
    event::run(ctx, event_loop, board);
}

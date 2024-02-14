#![allow(unused)]

use ggez::{ContextBuilder,
    conf::WindowMode,
    GameResult,
    event::{self, EventHandler},
    graphics::{self, Color, Rect, Mesh, DrawParam, Image},
    glam::vec2};

use chess_rs::piece::*;
use chess_rs::board::*;

const CELL_SIZE: (f32, f32) = (120.0, 120.0);
const GRID_SIZE: (f32, f32) = (8.0, 8.0);
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);

#[derive(Debug)]
struct State {
    board: Board,
}

impl State {
    fn new() -> Self {
        State {
            board: Board::new(),
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for i in 0..GRID_SIZE.0 as usize{
            for j in 0..GRID_SIZE.1 as usize{
                let color = if (i+j)%2 == 0 {Color::WHITE} else {Color::from_rgb(128, 128, 128)};
                let rect = Mesh::new_rectangle(ctx,
                                               graphics::DrawMode::fill(),
                                               Rect::new(
                                                   i as f32 * CELL_SIZE.0,
                                                   j as f32 * CELL_SIZE.1,
                                                   CELL_SIZE.0,
                                                   CELL_SIZE.1,
                                                   ), color)?;
                canvas.draw(&rect, DrawParam::default());

                let path = match &self.board.grid[j][i] {
                    Some(piece) => {
                        match (&piece.piece_type, &piece.piece_color) {
                            (PieceType::King {is_checkmated, in_check, has_moved}, PieceColor::White) => {"/white-king.png"},
                            (PieceType::Queen, PieceColor::White) => {"/white-queen.png"},
                            (PieceType::Bishop, PieceColor::White) => {"/white-bishop.png"},
                            (PieceType::Knight, PieceColor::White) => {"/white-knight.png"},
                            (PieceType::Rook {has_moved}, PieceColor::White) => {"/white-rook.png"},
                            (PieceType::Pawn {has_moved}, PieceColor::White) => {"/white-pawn.png"},
                            (PieceType::King {is_checkmated, in_check, has_moved}, PieceColor::Black) => {"/black-king.png"},
                            (PieceType::Queen, PieceColor::Black) => {"/black-queen.png"},
                            (PieceType::Bishop, PieceColor::Black) => {"/black-bishop.png"},
                            (PieceType::Knight, PieceColor::Black) => {"/black-knight.png"},
                            (PieceType::Rook {has_moved}, PieceColor::Black) => {"/black-rook.png"},
                            (PieceType::Pawn {has_moved}, PieceColor::Black) => {"/black-pawn.png"},
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
    let board = State::new();
    let (ctx, event_loop) = ContextBuilder::new("chess-rs", "Eyepatch")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;
    event::run(ctx, event_loop, board);
}

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::read_buffer::Player;

pub static WINDOW_WIDTH: u32 = 1000;
pub static WINDOW_HEIGHT: u32 = 900;

pub fn draw_board(canvas: &mut Canvas<Window>, player: Player) {
    // canvas.set_draw_color(Color::RGBA(206, 102, 56, 10));

    canvas.set_draw_color(Color::WHITE);
    let line_width = WINDOW_WIDTH / player.board.len() as u32;
    let line_height = WINDOW_HEIGHT / player.board[0].len() as u32;

    let mut intervale_w = 0;
    let mut intervale_h = 0;

    while intervale_w < WINDOW_WIDTH {
        canvas
            .draw_line(
                Point::new(intervale_w as i32, 0),
                Point::new(intervale_w as i32, WINDOW_HEIGHT as i32),
            )
            .unwrap();
        intervale_w += line_height;
    }

    while intervale_h < WINDOW_HEIGHT {
        canvas
            .draw_line(
                Point::new(0, intervale_h as i32),
                Point::new(WINDOW_WIDTH as i32, intervale_h as i32),
            )
            .unwrap();
        intervale_h += line_width;
    }

    for x in 0..player.board.len() {
        for y in 0..player.board[0].len() {
            if player.board[x][y] == player.identifiant.0
                || player.board[x][y] == player.identifiant.1
            {
                let rect = Rect::new(
                    x as i32 * line_width as i32,
                    y as i32 * line_height as i32,
                    line_width,
                    line_height,
                );
                canvas.set_draw_color(Color::BLUE);
                canvas.fill_rect(rect).unwrap();
            }
            if player.board[x][y] != player.identifiant.0
                && player.board[x][y] != player.identifiant.1
                && player.board[x][y] != '.'
            {
                let rect = Rect::new(
                    x as i32 * line_width  as i32,
                    y as i32 * line_height as i32,
                    line_width,
                    line_height,
                );
                canvas.set_draw_color(Color::RED);
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
}

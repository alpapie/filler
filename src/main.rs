mod read_buffer;
mod solver;
pub mod visialization;
use std::{sync::{Arc, Mutex}, time::Duration};

use read_buffer::{ Player};
use sdl2::{event::Event, keyboard::Keycode, libc::sleep};
use visialization::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    // read_loop();

        let mut player = Player::new();
    
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem
            .window("filler", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
    
        // let player_clone = &player);
        // std::thread::spawn(move || {
        //     read_loop(player_clone);
        // });
    
        'running: loop {
            canvas.clear();

            {
                player.add_identifiant();
                player.add_board();
                player.add_piece();
                // player.visialization(&mut canvas);
                player.solve();
                player.visialization(&mut canvas);
            }
            canvas.present();
    
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        }
        // std::thread::sleep(Duration::from_secs(30));
}
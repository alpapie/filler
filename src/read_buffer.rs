use std::{io, sync::{Arc, Mutex}, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump};

use crate::{solver::get_coor, visialization::{ draw_board, WINDOW_HEIGHT, WINDOW_WIDTH}};

// std::thread::sleep(Duration::from_secs(20))
#[derive(Debug,Clone)]
pub struct Player{
    pub identifiant :(char,char),
    pub board       : Vec<Vec<char>>,
    pub piece       : Vec<Vec<char>>
}

impl Player{
    pub fn new()->Self{
        Self{
            identifiant : (' ',' '),
            board :Vec::new(),
            piece :Vec::new(),
        }
    }

    pub fn add_identifiant(&mut self){
        // $$$ exec p1 : [solution/filler/target/debug/filler]
        if self.identifiant.1==' ' {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("error lors de la lecture");
            self.identifiant= if  buffer.contains("p1"){
                ('a','@')
            }else{
                ('s','$')
            }
        }
    }

    pub fn add_board(&mut self){
        // Anfield 40 30:

        let mut buffer = String::new();
        if  io::stdin().read_line(&mut buffer).is_err() || buffer.is_empty() {
            std::thread::sleep(Duration::from_secs(10));
        }
        // io::stdin().read_line(&mut buffer).expect("error lors de la lecture");
        let row= buffer.replace(":", "").replace("Anfield ", "").split_whitespace().map(|elem| elem.parse().unwrap()).collect::<Vec<u32>>()[1];
        let _=io::stdin().read_line(&mut buffer);
        self.board=[].to_vec();
        for _ in 0..row{
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("error lors de la lecture");
            let list_char= line.split(" ").collect::<Vec<_>>()[1].replace("\n", "").chars().collect::<Vec<char>>();
            self.board.push(list_char);
        }
    }

    pub fn add_piece(&mut self){
        // Piece 5 6:
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("error lors de la lecture");
        let row= buffer.replace(":", "").replace("Piece ", "").split_whitespace().map(|elem| elem.parse().unwrap()).collect::<Vec<u32>>()[1];
        self.piece=[].to_vec();
        for _ in 0..row{
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("error lors de la lecture");
            let list_piece= line.replace("\n", "").chars().collect::<Vec<char>>();
            self.piece.push(list_piece);
        }
    }
    pub fn solve(&self){
        let coord= get_coor(self);
        println!("{} {}",coord.0,coord.1);
    }
    pub fn visialization(&self, canvas: &mut Canvas<Window>){
        canvas.clear();
        draw_board(canvas, self.clone());
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
        canvas.present();
       
    }
    
}

// let mut player_lock = player.lock().unwrap();
// player_lock.add_identifiant();
// player_lock.add_board();
// player_lock.add_piece();
// drop(player_lock); // Release lock before visualizing

// let player_lock = player.lock().unwrap();
// player_lock.solve();

// pub fn read_loop( ) {
//    let mut player = Player::new();
    
//     loop {
//         player.add_identifiant();
//         player.add_board();
//         player.add_piece();
//         // player.visialization(&mut canvas);
//         player.solve();
//     }
    

// }

// pub fn read_loop(player: Arc<Mutex<Player>>) {
//     loop {
//         let mut player_lock = player.lock().unwrap();
//         player_lock.add_identifiant();
//         player_lock.add_board();
//         player_lock.add_piece();
//         drop(player_lock); // Release lock before visualizing

//         let player_lock = player.lock().unwrap();
//         player_lock.solve();
//     }
// }
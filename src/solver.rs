use crate::read_buffer::Player;

pub fn get_coor(player: &Player) -> (u32, u32) {
    let enemy_positions = find_enemy_positions(&player.board, player.identifiant);
    let mut possible_moves = Vec::new();

    for x in 0..player.board.len() {
        for y in 0..player.board[0].len() {
            let safe = can_place(x, y, &player.board, &player.piece, player.identifiant);
            if safe {
                possible_moves.push((x as i32, y as i32));
            }
        }
    }

    if !possible_moves.is_empty() && !enemy_positions.is_empty() {
        let result = attack_enemy(possible_moves, &enemy_positions, &player.board, player.identifiant);
        return (result.1 as u32, result.0 as u32);
    }
    (0, 0)
}

fn attack_enemy(possible_moves: Vec<(i32, i32)>, enemy_positions: &[(usize, usize)], board: &Vec<Vec<char>>, player_symbol: (char, char)) -> (i32, i32) {
    let mut best_move = possible_moves[0];

    for &coord in possible_moves.iter() {
        if is_preferred(coord, board, player_symbol) {
            let min_distance = enemy_positions.iter()
                .map(|&enemy_pos| distance(coord, (enemy_pos.0 as i32, enemy_pos.1 as i32)))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            let current_min_distance = enemy_positions.iter()
                .map(|&enemy_pos| distance(best_move, (enemy_pos.0 as i32, enemy_pos.1 as i32)))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            if min_distance < current_min_distance {
                best_move = coord;
            }
        }
    }
    best_move
}

fn is_preferred(coord: (i32, i32), board: &Vec<Vec<char>>, player_symbol: (char, char)) -> bool {
    let (x, y) = coord;
    if x > 0 && (board[(x - 1) as usize][y as usize] == player_symbol.0 || board[(x - 1) as usize][y as usize] == player_symbol.1) {
        return false;
    }
    if y > 0 && (board[x as usize][(y - 1) as usize] == player_symbol.0 || board[x as usize][(y - 1) as usize] == player_symbol.1) {
        return false;
    }
    if x < (board.len() as i32) - 1 && (board[(x + 1) as usize][y as usize] == player_symbol.0 || board[(x + 1) as usize][y as usize] == player_symbol.1) {
        return false;
    }
    if y < (board[0].len() as i32) - 1 && (board[x as usize][(y + 1) as usize] == player_symbol.0 || board[x as usize][(y + 1) as usize] == player_symbol.1) {
        return false;
    }
    true
}

pub fn distance(current: (i32, i32), enemy: (i32, i32)) -> f32 {
    let dx = (enemy.0 - current.0) as f32;
    let dy = (enemy.1 - current.1) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub fn can_place(x: usize, y: usize, board: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, player_symbol: (char, char)) -> bool {
    let mut overlap = 0;
    let piece_height = piece.len();
    let piece_width = piece[0].len();
    let board_height = board.len();
    let board_width = board[0].len();

    for i in 0..piece_height {
        for j in 0..piece_width {
            let board_x = x + i;
            let board_y = y + j;
            if board_x >= board_height || board_y >= board_width {
                return false;
            }
            let board_char = board[board_x][board_y];
            if piece[i][j] != '.' {
                if board_char == player_symbol.0 || board_char == player_symbol.1 {
                    overlap += 1;
                } else if board_char != '.' {
                    return false;
                }
            }
        }
    }
    overlap == 1
}

pub fn find_enemy_positions(board: &Vec<Vec<char>>, player_symbol: (char, char)) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            let cell = board[x][y];
            if cell != '.' && cell != player_symbol.0 && cell != player_symbol.1 {
                positions.push((x, y));
            }
        }
    }
    positions
}

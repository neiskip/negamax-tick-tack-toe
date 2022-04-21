use crate::board::Board;
use crate::strategy::negamax;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use std::io::Write;
use std::os::raw::c_char;

pub trait Moove{
    fn turn(&self, c_choice : &String, h_choice: &String, board: &mut Board) -> bool;
}
#[derive(Debug,Clone, PartialEq, Eq, Hash, Copy)]
pub enum Player{
    Human,
    Computer,
    None
}

impl Moove for Player {
    fn turn(&self, c_choice: &String, h_choice: &String, board: &mut Board) -> bool {
        match *self {
            Player::Human => Self::h_turn(c_choice, h_choice, board),
            Player::Computer => Self::c_turn(c_choice, h_choice, board),
            Player::None => false
        }
    }

}

impl Player {
    fn h_turn(c_choice: &String, h_choice: &String, board: &mut Board) -> bool {
        let depth = board.empty_cells().len();
        if depth == 0 || board.game_over(){
            return false;
        }

        let mut moove = String::from("-1");
        let mooves: HashMap<_,_> = [
            (1, [0, 0]), (2, [0, 1]), (3, [0, 2]),
            (4, [1, 0]), (5, [1, 1]), (6, [1, 2]),
            (7, [2, 0]), (8, [2, 1]), (9, [2, 2]),
        ].into_iter().collect();

        board.clean();
        println!("Ход человека {}", h_choice);
        board.render(c_choice, h_choice);

        while moove.parse::<i32>().unwrap() < 1 || moove.parse::<i32>().unwrap() > 9 {
            println!("Выберите клетку (1..9)");
            let _ = std::io::stdout().flush();
            moove = "".to_string();
            std::io::stdin().read_line(&mut moove).expect("Не верный ввод");
            if let Some('\n')=moove.chars().next_back() {
                moove.pop();
            }
            if let Some('\r')=moove.chars().next_back() {
                moove.pop();
            }
            let coord = mooves.get(&moove.parse::<i32>().unwrap()).unwrap();
            if !board.set_move(coord[0], coord[1], Player::Human){
                println!("Не верный ход");
                moove = "-1".to_string();
            }

        }
        true
    }
    fn c_turn(c_choice: &String, h_choice: &String, board: &mut Board) -> bool {
        let depth = board.empty_cells().len();
        if depth == 0 || board.game_over(){
            return false;
        }
        board.clean();
        println!("Ход компьютера [{}]", c_choice);
        board.render(c_choice, h_choice);
        let x; let y;
        if depth == 9{
            x = *[1,2,3].to_vec().choose(&mut rand::thread_rng()).unwrap();
            y = *[1,2,3].to_vec().choose(&mut rand::thread_rng()).unwrap();
        } else {
            let moved = negamax(board, depth as i32, &Player::Computer);
            x = moved[0];
            y = moved[1];
        }

        board.set_move(x as usize,y as usize, Player::Computer)
    }
}
/*
impl Moove for Player::Human{
     fn turn(&self, c_choice : &String, h_choice: &String, board: &mut Board) -> bool{
         let depth = board.empty_cells().len();
         if depth == 0 || board.game_over(){
             return false;
         }

         let mut moove = String::from("-1");
         let mooves: HashMap<_,_> = [
             (1, [0, 0]), (2, [0, 1]), (3, [0, 2]),
             (4, [1, 0]), (5, [1, 1]), (6, [1, 2]),
             (7, [2, 0]), (8, [2, 1]), (9, [2, 2]),
         ].into_iter().collect();

         board.clean();
         println!("Ход человека {}", h_choice);
         board.render(c_choice, h_choice);

         while moove.parse::<i32>().unwrap() < 1 || moove.parse::<i32>().unwrap() > 9 {
             println!("Выберите клетку (1..9)");
             let _ = std::io::stdout().flush();
             moove = "".to_string();
             std::io::stdin().read_line(&mut moove).expect("Не верный ввод");
             if let Some('\n')=moove.chars().next_back() {
                 moove.pop();
             }
             if let Some('\r')=moove.chars().next_back() {
                 moove.pop();
             }
             let coord = mooves.get(&moove.parse::<i32>().unwrap()).unwrap();
             if !board.set_move(coord[0], coord[1], "h"){
                 println!("Не верный ход");
                 moove = "-1".to_string();
             }

         }
         true
    }
}

impl Moove for Player::Computer{
    fn turn(c_choice : &String, h_choice: &String, board: &mut Board) -> bool{
        let depth = board.empty_cells().len();
        if depth == 0 || board.game_over(){
            return false;
        }
        board.clean();
        println!("Ход компьютера [{}]", c_choice);
        board.render(c_choice, h_choice);
        let x; let y;
        if depth == 9{
            x = *[1,2,3].to_vec().choose(&mut rand::thread_rng()).unwrap();
            y = *[1,2,3].to_vec().choose(&mut rand::thread_rng()).unwrap();
        } else {
            let moved = negamax(board, depth as i32, "c");
            x = moved[0];
            y = moved[1];
        }

        board.set_move(x as usize,y as usize, "c")
    }
}
*/
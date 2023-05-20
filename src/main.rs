use std::thread;
use std::time::Duration;

mod board;
mod atype;
mod direction;
mod coordination;
mod ant;

pub use crate::board::Board;
pub use crate::atype::Atype;
pub use crate::direction::Direction;
pub use crate::coordination::Coordination;

fn main() {
    let mut board = Board
    {
        size: 100,
        coordinations: Vec::new(),
        ants: Vec::new(),
    };

    board.fill_coordination_list();
    let mut rng = rand::thread_rng();
    board.drop_the_ant(&mut rng, 1, Atype::Black);
    //board.drop_the_ant(&mut rng, 1, Atype::Red);
    let mut i = 0;
    loop {
        i += 1;
        println!("{}", i);
        board.make_turn();
        board.print_the_board();
        //thread::sleep(Duration::from_millis(100));
    }

}

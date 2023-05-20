pub use crate::atype::Atype;
pub use crate::direction::Direction;
pub use crate::coordination::Coordination;

pub struct Ant {
    pub position: Coordination,
    pub direction: Direction,
    pub ant_Atype: Atype,
    //pub moves: Vec<&'static str>,
}

impl Ant {
    pub fn make_move(&mut self, s: i32) {
        match self.direction {
            Direction::South => {
                self.position.x = ((self.position.x + 1) % s + s) % s;
                //self.moves.push("South");
            }
            Direction::North => {
                self.position.x = ((self.position.x - 1) % s + s) % s;
                //self.moves.push("North");
            }
            Direction::West => {
                self.position.y = ((self.position.y - 1) % s + s) % s;
                //self.moves.push("West");
            }
            Direction::East => {
                self.position.y = ((self.position.y + 1) % s + s) % s;
                //self.moves.push("East");
            }
        }
        println!("{}, {}", self.position.x, self.position.y);
    }
}

pub use crate::ant::Ant;
pub use crate::atype::Atype;
pub use crate::direction::Direction;
pub use crate::coordination::Coordination;

use rand::Rng;

pub struct Board {
    pub size: u32,
    pub coordinations: Vec<Coordination>,
    pub ants: Vec<Ant>,
}

impl Board {
   pub fn fill_coordination_list(&mut self) {
        self.coordinations.clear();
        for i in 0..self.size {
            for j in 0..self.size {
                self.coordinations.push(Coordination {
                    x: i as i32,
                    y: j as i32,
                    sign: Atype::White,
                });
            }
        }
    }
    
    pub fn print_the_board(&mut self) {
        let mut index = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{}", self.coordinations[index].sign);
                index += 1;
            }
            println!();
        }
        println!();
    }

    pub fn drop_the_ant(&mut self, rng: &mut impl Rng, ant_number: u8, ant: Atype) {
        let mut index: u8 = 0;
        while index < ant_number {
            let rn_x = rng.gen_range(0..self.size) as i32;
            let rn_y = rng.gen_range(0..self.size) as i32;
            self.ants.push(Ant {position: Coordination {x: rn_x, y: rn_y, sign: Atype::White}, direction: Direction::North, ant_Atype: ant.clone()/*, moves: Vec::new()*/});
            index += 1;
        }
    }

    /*pub fn list_has_cycle<T: PartialEq>(&mut self, vec: &[T]) -> bool {
        if vec.len() < 5 {
            return false;
        }
    
        let mut tortoise = 0;
        let mut hare = 0;
    
        loop {
            if hare >= vec.len() || vec[hare] == vec[tortoise] {
                return hare < vec.len();
            }
    
            tortoise += 1;
            hare += 1;
    
            if hare >= vec.len() || vec[hare] == vec[tortoise] {
                return hare < vec.len();
            }
    
            hare += 1;
        }
    }*/

    pub fn make_turn(&mut self) {
        let s = self.size as i32;

        let mut ants_to_remove: Vec<Ant> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for ant in self.ants.iter() {
            if ant.ant_Atype == Atype::Black {
                let ant_x = ant.position.x;
                let ant_y = ant.position.y;
                
                let should_remove = self.ants.iter().any(|other_ant| {
                    other_ant.ant_Atype == Atype::Red &&
                    other_ant.position.x >= ant_x - 2 && other_ant.position.x <= ant_x + 2 &&
                    other_ant.position.y >= ant_y - 2 && other_ant.position.y <= ant_y + 2
                });

                if should_remove {
                    println!("Anttoremove: {}, {},", ant.position.x, ant.position.y);
                    x = ant.position.x;
                    y = ant.position.y;
                }
            }
        }
        self.ants.retain(|ant| {
            x != ant.position.x || y != ant.position.y
        });

        for ant in self.ants.iter() {
            println!("Ant: {}, {},", ant.position.x, ant. position.y);
        }
        for ant in self.ants.iter_mut() {
            ant.make_move(s);
            for coord in self.coordinations.iter_mut() {
                if coord.x == ant.position.x && coord.y == ant.position.y && ant.ant_Atype == Atype::Black{
                    
                    coord.sign = match &mut coord.sign {
                        Atype::White => Atype::Black,
                        Atype::Black => Atype::White,
                        Atype::Red => Atype::White,
                        _ => Atype::White,
            
                    };
                    ant.direction = match (&mut coord.sign, &mut ant.direction) {
                        //czarna mrówka - białe w prawo
                        (Atype::White, Direction::South) => Direction::West,
                        (Atype::White, Direction::North) => Direction::East,
                        (Atype::White, Direction::West) => Direction::North,
                        (Atype::White, Direction::East) => Direction::South,
                        //czarna mrówka - czarne w lewo
                        (Atype::Black, Direction::South) => Direction::East,
                        (Atype::Black, Direction::North) => Direction::West,
                        (Atype::Black, Direction::West) => Direction::South,
                        (Atype::Black, Direction::East) => Direction::North,
                        //czarna mrówka - czerwone ucieczka
                        (Atype::Red, Direction::South) => Direction::North,
                        (Atype::Red, Direction::North) => Direction::South,
                        (Atype::Red, Direction::West) => Direction::East,
                        (Atype::Red, Direction::East) => Direction::West,
                        _ => ant.direction,
                    };
                }
                if coord.x == ant.position.x && coord.y == ant.position.y && ant.ant_Atype == Atype::Red {
                    coord.sign = match &mut coord.sign {
                        Atype::Black => Atype::Red,
                        Atype::White => Atype::Red,
                        Atype::Red => Atype::White,
                        _ => Atype::White,
            
                    };
                    ant.direction = match (&mut coord.sign, &mut ant.direction) {
                        //czerwona mrówka - białe w lewo
                        (Atype::White, Direction::South) => Direction::East,
                        (Atype::White, Direction::North) => Direction::West,
                        (Atype::White, Direction::West) => Direction::South,
                        (Atype::White, Direction::East) => Direction::North,
                        //czerwona mrówka - czerwone w prawo
                        (Atype::Red, Direction::South) => Direction::West,
                        (Atype::Red, Direction::North) => Direction::East,
                        (Atype::Red, Direction::West) => Direction::North,
                        (Atype::Red, Direction::East) => Direction::South,
                        //czerwona mrówka - czarne prosto
                        (Atype::Black, Direction::South) => Direction::South,
                        (Atype::Black, Direction::North) => Direction::North,
                        (Atype::Black, Direction::West) => Direction::West,
                        (Atype::Black, Direction::East) => Direction::East,

                        _ => ant.direction,
                    };
                }
            }
        }
        /*let mut index = 0;
        while index < self.ants.len() {
            let moves_copy = self.ants[index].moves.clone();
            if self.list_has_cycle(&moves_copy) {
                println!("Mrówka wpadła w cykl!!!");
            }
            index += 1;
        }*/
    }
}
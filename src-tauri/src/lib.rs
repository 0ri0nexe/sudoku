use rand::{self, Rng};
use std::{fmt::Display, vec};
#[derive(Debug)]
pub struct Sudoku {
    board: [u8; 81],
}

impl Sudoku {
    pub fn new(random: bool) -> Sudoku {
        let mut sudoku = Sudoku { board: [0u8; 81] };
        if !random {
            return sudoku;
        }
        let mut rng = rand::thread_rng();

        //setup the first block
        for x in 0..9 {
            let y = 1u8;
            sudoku.set_random_number(x, y, &mut rng);
        }
        for y in 0..9 {
            let x = 1u8;
            sudoku.set_random_number(x, y, &mut rng);
        }
        let solved_sudoku = solve(&sudoku);
        match solved_sudoku {
            None => panic!("problem during creation of the sudoku"),
            Some(sudoku) => return sudoku,
        }
    }

    pub fn set_random_number(&mut self, x: u8, y: u8, rng: &mut rand::rngs::ThreadRng) {
        loop {
            let mut possibilities: Vec<u8> = (1..=9).collect();
            let chosen_possibility: usize = rng.gen_range(0..possibilities.len());
            self.set(
                &Coords { x, y },
                *possibilities.get(chosen_possibility).unwrap(),
            );
            if self.is_in_rule(&Coords { x, y }) {
                break;
            }
            possibilities.remove(chosen_possibility);

            // SHOULD NEVER BE REACHED
            if possibilities.len() == 0 {
                panic!("problem during the creation of the new sudoku");
            }
        }
    }

    pub fn get_playable_sudoku(self, difficulty: f64) -> Sudoku {
        let solved_sudoku = solve(&self);
        if let None = solved_sudoku {
            return self;
        }

        let mut solved_sudoku = solved_sudoku.unwrap();
        let numbers_to_remove = (81 - 7) as f64 * (1 as f64 - difficulty);
        let mut numbers: Vec<i32> = (0..81).collect();
        let mut rng = rand::thread_rng();
        for _ in 0..(numbers_to_remove as i32) {
            let num_choice = rng.gen_range(0..numbers.len());
            let coords_1d = *numbers.get(num_choice).unwrap();
            let coords = Coords::from(coords_1d);
            numbers.remove(num_choice);
            solved_sudoku.set(&coords, 0);
        }
        return solved_sudoku;
    }

    pub fn from(board: [u8; 81]) -> Sudoku {
        for i in board {
            assert!(i <= 9)
        }
        Sudoku { board }
    }

    pub fn board(&self) -> &[u8; 81] {
        &self.board
    }

    pub fn get(&self, coords: &Coords) -> u8 {
        self.board()[coords.to_int()]
    }

    pub fn set(&mut self, coords: &Coords, value: u8) {
        self.board[coords.to_int()] = value;
    }

    pub fn is_in_rule(&self, coords: &Coords) -> bool {
        if self.vertical_check(coords) && self.horizontal_check(coords) && self.block_check(coords)
        {
            return true;
        }
        false
    }

    pub fn vertical_check(&self, coords: &Coords) -> bool {
        let current_value = self.get(&coords);
        if current_value == 0 {
            return true;
        }

        for i in 0..9 {
            if (i != coords.x) && (self.get(&Coords { x: i, y: coords.y }) == current_value) {
                return false;
            }
        }
        true
    }

    pub fn horizontal_check(&self, coords: &Coords) -> bool {
        let current_value = self.get(&coords);
        if current_value == 0 {
            return true;
        }

        for i in 0..9 {
            if (i != coords.y) && (self.get(&Coords { x: coords.x, y: i }) == current_value) {
                return false;
            }
        }
        true
    }

    pub fn block_check(&self, coords: &Coords) -> bool {
        let current_value = self.get(coords);
        if current_value == 0 {
            return true;
        }

        let block_coords = Coords {
            x: coords.x / 3,
            y: coords.y / 3,
        };
        for i in 0..3 {
            for j in 0..3 {
                let coords_to_check = Coords {
                    x: block_coords.x * 3 + i,
                    y: block_coords.y * 3 + j,
                };
                if coords_to_check == *coords {
                    continue;
                }
                if self.get(&coords_to_check) == current_value {
                    return false;
                }
            }
        }
        true
    }

    pub fn to_array(&self) -> [u8; 81] {
        self.board
    }

    pub fn to_vector(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.board.map(|value| result.push(value as i32));
        result
    }
}

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Sudoku { board: self.board }
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_write = String::new();
        for (i, value) in self.board.iter().enumerate() {
            if i % 9 == 0 {
                to_write += "\n"
            }
            to_write += " ";
            to_write += &value.to_string();
        }
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, PartialEq)]
pub struct Coords {
    pub x: u8,
    pub y: u8,
}

impl Coords {
    fn from(coord_1d: i32) -> Coords {
        return Coords {
            x: (coord_1d % 9) as u8,
            y: (coord_1d / 9) as u8,
        };
    }

    fn to_int(&self) -> usize {
        (self.x + self.y * 9) as usize
    }

    fn inc(&mut self) {
        if self.x == 8 {
            if self.x == 0 {
                panic!("x cannot be negative");
            }
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
    }
    fn dec(&mut self) {
        if self.x == 0 {
            if self.y == 0 {
                panic!("y cannot be negative");
            }
            self.y -= 1;
            self.x = 8;
        } else {
            self.x -= 1;
        }
    }
}

pub fn solve(sudoku: &Sudoku) -> Option<Sudoku> {
    for x in 0..9 {
        for y in 0..9 {
            if !sudoku.is_in_rule(&Coords {
                x: x as u8,
                y: y as u8,
            }) {
                return None;
            }
        }
    }

    let mut current_search = Coords { x: 0, y: 0 };
    let mut solver_sudoku = sudoku.clone();
    let mut go_back = false;

    loop {
        if solver_sudoku.get(&current_search) == 0 {
            solver_sudoku.set(&current_search, 1);
        }

        if go_back {
            if current_search.x == 0 && current_search.y == 0 {
                return None;
            }

            if solver_sudoku.get(&current_search) != sudoku.get(&current_search) {
                solver_sudoku.set(&current_search, 0);
            }
            current_search.dec();

            if solver_sudoku.get(&current_search) != sudoku.get(&current_search) {
                let current_value = solver_sudoku.get(&current_search);

                if current_value < 9 {
                    go_back = false;
                    solver_sudoku.set(&current_search, current_value + 1);
                }
            }
            continue;
        }

        let value = solver_sudoku.get(&current_search);

        if value == sudoku.get(&current_search) {
            if current_search.x == 8 && current_search.y == 8 {
                return Some(solver_sudoku);
            }
            current_search.inc();
            continue;
        }

        if solver_sudoku.is_in_rule(&current_search) {
            if current_search.x == 8 && current_search.y == 8 {
                return Some(solver_sudoku);
            }
            current_search.inc();
            continue;
        }

        if value == 9 {
            go_back = true;
            continue;
        }
        solver_sudoku.set(&current_search, value + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_operations() {
        let mut coords = Coords { x: 8, y: 0 };
        coords.inc();
        assert_eq!(coords, Coords { x: 0, y: 1 });
    }

    #[test]
    fn board_creation() {
        let sudoku = Sudoku::new(false);
        let board = sudoku.board();

        assert_eq!(board, &[0u8; 81]);

        let sudoku = Sudoku::from([0u8; 81]);
        let board = sudoku.board();
        assert_eq!(board, &[0u8; 81]);
    }

    #[test]
    fn coords() {
        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);
        assert_eq!(sudoku.get(&Coords { x: 0, y: 0 }), 5u8);
        assert_eq!(sudoku.get(&Coords { x: 5, y: 5 }), 2u8);
        assert_eq!(sudoku.get(&Coords { x: 0, y: 8 }), 7u8);
    }

    #[test]
    fn vertical_rule() {
        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);

        let coords = [
            Coords { x: 0, y: 0 },
            Coords { x: 8, y: 2 },
            Coords { x: 4, y: 6 },
            Coords { x: 1, y: 5 },
            Coords { x: 4, y: 7 },
            Coords { x: 3, y: 1 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.vertical_check(&coord),
                true,
                "\ntest failed at coords {:#?}\nexpected true",
                coord
            );
        }

        let sudoku = Sudoku::from([
            5u8, 0, 5, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 1, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 9, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 3, 2, 0, 0, 0, 6, 2, 3,
        ]);

        let coords = [
            Coords { x: 2, y: 0 },
            Coords { x: 0, y: 2 },
            Coords { x: 1, y: 6 },
            Coords { x: 2, y: 8 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.vertical_check(&coord),
                false,
                "\ntest failed at coords {:#?}\nexpected false",
                coord
            );
        }
    }

    #[test]
    fn horizontal_rule() {
        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);

        let coords = [
            Coords { x: 0, y: 0 },
            Coords { x: 8, y: 2 },
            Coords { x: 4, y: 6 },
            Coords { x: 1, y: 5 },
            Coords { x: 4, y: 7 },
            Coords { x: 3, y: 1 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.horizontal_check(&coord),
                true,
                "\ntest failed at coords {:#?}\nexpected true",
                coord
            );
        }

        let sudoku = Sudoku::from([
            5u8, 0, 8, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 7, 0, 2, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 4, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);

        let coords = [
            Coords { x: 2, y: 0 },
            Coords { x: 0, y: 2 },
            Coords { x: 1, y: 6 },
            Coords { x: 2, y: 8 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.horizontal_check(&coord),
                false,
                "\ntest failed at coords {:#?}\nexpected false",
                coord
            );
        }
    }

    #[test]
    fn block_rule() {
        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);
        let coords = [
            Coords { x: 0, y: 0 },
            Coords { x: 5, y: 5 },
            Coords { x: 8, y: 3 },
            Coords { x: 0, y: 8 },
        ];

        for coord in coords {
            assert_eq!(
                sudoku.block_check(&coord),
                true,
                "\ntest failed at coords {:#?}\nexpected true",
                coord
            );
        }

        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 5, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 8, 6, 0, 3, 5, 0, 7, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 7, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);
        let coords = [
            Coords { x: 0, y: 0 },
            Coords { x: 4, y: 4 },
            Coords { x: 8, y: 3 },
            Coords { x: 0, y: 8 },
        ];

        for coord in coords {
            assert_eq!(
                sudoku.block_check(&coord),
                false,
                "\ntest failed at coords {:#?}\nexpected false",
                coord
            );
        }
    }

    #[test]
    fn rule() {
        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);

        let coords = [
            Coords { x: 0, y: 0 },
            Coords { x: 8, y: 2 },
            Coords { x: 4, y: 6 },
            Coords { x: 1, y: 5 },
            Coords { x: 4, y: 7 },
            Coords { x: 3, y: 1 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.is_in_rule(&coord),
                true,
                "\ntest failed at coords {:#?}\nexpected true\n",
                coord
            );
        }

        let sudoku = Sudoku::from([
            5u8, 0, 5, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 1, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 9, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 3, 2, 0, 0, 0, 6, 2, 3,
        ]);

        let coords = [
            Coords { x: 2, y: 0 },
            Coords { x: 0, y: 2 },
            Coords { x: 1, y: 6 },
            Coords { x: 2, y: 8 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.is_in_rule(&coord),
                false,
                "\ntest failed at coords {:#?}\nexpected false",
                coord
            );
        }

        let sudoku = Sudoku::from([
            5u8, 0, 8, 0, 0, 0, 9, 0, 0, 0, 0, 4, 6, 9, 0, 0, 0, 0, 7, 0, 2, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 0, 6, 0, 3, 5, 0, 0, 2, 0, 0, 0, 4, 4, 0,
            7, 0, 0, 0, 9, 5, 0, 0, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);

        let coords = [
            Coords { x: 2, y: 0 },
            Coords { x: 0, y: 2 },
            Coords { x: 1, y: 6 },
            Coords { x: 2, y: 8 },
        ];
        for coord in coords {
            assert_eq!(
                sudoku.is_in_rule(&coord),
                false,
                "\ntest failed at coords {:#?}\nexpected false",
                coord
            );
        }

        let sudoku = Sudoku::from([
            5u8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 5, 4, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 6, 2, 0,
            4, 7, 3, 6, 0, 2, 5, 8, 0, 9, 8, 1, 7, 5, 4, 8, 6, 0, 3, 5, 0, 7, 2, 0, 0, 0, 4, 0, 0,
            7, 0, 0, 0, 9, 5, 0, 7, 6, 0, 0, 1, 0, 0, 0, 7, 0, 2, 0, 0, 0, 6, 0, 3,
        ]);
        let coords = [
            Coords { x: 0, y: 0 },
            Coords { x: 4, y: 4 },
            Coords { x: 8, y: 3 },
            Coords { x: 0, y: 8 },
        ];

        for coord in coords {
            assert_eq!(
                sudoku.is_in_rule(&coord),
                false,
                "\ntest failed at coords {:#?}\nexpected false",
                coord
            );
        }
    }
}

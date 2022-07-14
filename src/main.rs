use std::io;

use crate::game_core::game_state_builder::GameStateBuilder;
use crate::game_core::minefield::{CellType, Minefield};
use crate::game_core::cell_opener::{OpenResult, CellOpener};

mod game_core;


fn print_field(minefield: &Minefield) {
    for y in 0..minefield.height() {
        for x in 0..minefield.width() {
            let cell = minefield.cell((x, y));
            match cell.cell_type {
                CellType::Mine => print!("m"),
                CellType::Counter(counter) => print!("{}", counter)
            }
        }
        print!("\n");
    }
}

fn print_with_fog_of_war(minefield: &Minefield) {
    for y in 0..minefield.height() {
        for x in 0..minefield.width() {
            let cell = minefield.cell((x, y));
            if cell.open {
                match cell.cell_type {
                    CellType::Mine => print!("m"),
                    CellType::Counter(counter) => print!("{}", counter)
                }
            } else {
                print!("x");
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut game_state = GameStateBuilder::make_game_state(10, 10, 10);
    let mut mine_field = game_state.mine_field_mut();

    print_field(&mine_field);

    loop {
        let mut command = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut command).unwrap();

        if command.trim() == "quit" {
            break;
        }

        let coords = command.split(" ").collect::<Vec<&str>>();
        let x = coords[0].trim().parse::<usize>().unwrap();
        let y = coords[1].trim().parse::<usize>().unwrap();

        let result = CellOpener::open_cell(&mut mine_field, (x, y));
        print_with_fog_of_war(&mine_field);

        if let OpenResult::Fail = result {
            println!("Btoooom!");
            break;
        }
    }
}

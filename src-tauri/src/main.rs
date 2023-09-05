// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde;
use sudoku::Sudoku;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(serde::Deserialize, serde::Serialize)]
struct Payload {
    sudoku: Vec<i32>,
}

#[tauri::command]
async fn get_new_sudoku() -> Payload {
    let sudoku = Sudoku::new(true).get_playable_sudoku(0.2);
    Payload {
        sudoku: sudoku.to_vector(),
    }
}

#[tauri::command]
async fn solve_sudoku(received_sudoku: Payload) -> Payload {
    // for now the payload wrapper is useless
    let received_sudoku = received_sudoku.sudoku;
    // putting sudoku in the right format
    assert!(received_sudoku.len() == 81);
    let mut new_sudoku: [u8; 81] = [0; 81];
    for (i, &value) in received_sudoku.iter().enumerate() {
        new_sudoku[i] = value as u8;
    }

    let received_sudoku = Sudoku::from(new_sudoku);

    let solved_sudoku = sudoku::solve(&received_sudoku);

    match solved_sudoku {
        None => {
            return Payload {
                sudoku: received_sudoku.to_vector(),
            };
        }
        Some(solved_sudoku) => {
            let mut result = Vec::new();
            for i in solved_sudoku.to_array() {
                result.push(i as i32);
            }
            return Payload { sudoku: result };
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![solve_sudoku, get_new_sudoku])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

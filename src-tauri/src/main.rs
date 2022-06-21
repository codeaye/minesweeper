#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod game;
mod random;
use std::sync::Mutex;

use game::{Minesweeper, MinesweeperView};

struct MineState(Mutex<Minesweeper>);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            view, open, flag, flag_count, reset, won, lost
        ])
        .manage(MineState(Mutex::new(Minesweeper::new(10, 10, 10))))
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

#[tauri::command]
fn open(x: usize, y: usize, state: tauri::State<MineState>) -> Vec<MinesweeperView> {
    let mut game = state.0.lock().unwrap();
    game.open((x, y));
    game.view()
}

#[tauri::command]
fn flag(x: usize, y: usize, state: tauri::State<MineState>) -> Vec<MinesweeperView> {
    let mut game = state.0.lock().unwrap();
    game.toggle_flag((x, y));
    game.view()
}

#[tauri::command]
fn reset(state: tauri::State<MineState>) -> Vec<MinesweeperView> {
    let mut game = state.0.lock().unwrap();
    game.reset();
    game.view()
}

#[tauri::command]
fn flag_count(state: tauri::State<MineState>) -> usize {
    state.0.lock().unwrap().flag_count()
}
#[tauri::command]
fn view(state: tauri::State<MineState>) -> Vec<MinesweeperView> {
    state.0.lock().unwrap().view()
}

#[tauri::command]
fn won(state: tauri::State<MineState>) -> bool {
    state.0.lock().unwrap().won()
}

#[tauri::command]
fn lost(state: tauri::State<MineState>) -> bool {
    state.0.lock().unwrap().lost()
}

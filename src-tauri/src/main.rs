#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    codroid_lib::run();
}

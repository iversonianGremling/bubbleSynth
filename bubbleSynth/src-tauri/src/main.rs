// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::atomic::{AtomicI32, Ordering}, os::windows::process};
use rodio::Sample;
use tauri::State;
use rand::Rng;

#[tauri::command]
fn generate_sound() -> Vec<f32> {
  let size = 1;
  let mut result : Vec<f32> = vec![0.0; size]; //Create empty vector
  let mut rng = rand::thread_rng();

  for _ in result.clone().iter()
  {
    let mut processed_sample = rng.gen_range(0.0..1.0);
    processed_sample *= 2.0;
    processed_sample -= 1.0;
    result.push(processed_sample);
  }

  result //Return the vector
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![generate_sound])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

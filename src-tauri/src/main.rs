#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;
use std::path::{PathBuf};
mod nd_conv;
use nd_conv::asl_conv;

fn main() -> std::io::Result<()> {
  let path: PathBuf = env::current_dir()?;
  println!("starting dir: {}", path.display());

  asl_conv::read_csv();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    Ok(())
}

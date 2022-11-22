#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod nd_conv;
use nd_conv::asl_conv;

fn main() {
  nd_conv::asl_conv::read_csv();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#![cfg_attr(
  	all(not(debug_assertions), target_os = "windows"),
  	windows_subsystem = "windows"
)]

use std::{sync::Arc, borrow::BorrowMut};

use tauri::async_runtime::Mutex;
use zzgengine::{engine::Engine, controllers::{Session, start_game_controller}};

type SharedAppState = Arc<Mutex<AppState>>;

struct AppState {
  	pub engine: Engine<0>
}

impl Session for AppState {
    fn set_engine(&mut self, engine: Engine<0>) {
        self.engine = engine
    }

    fn get_engine(&self) -> &Engine<0> {
        &self.engine
    }

    fn get_engine_mut(&mut self) -> &mut Engine<0> {
        &mut self.engine
    }
}

async fn get_session() -> Session {
    
}

#[tauri::command]
async fn start_game(shared_state: tauri::State<'_, SharedAppState>) {
    let mut state = shared_state.lock().await;
    start_game_controller::<AppState>(&mut state);
}

fn main() {
  	tauri::Builder::default()
		.manage(Arc::new(Mutex::new(AppState { engine: Engine::new() })))
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

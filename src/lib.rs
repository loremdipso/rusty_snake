mod game;
use game::game::Game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	wasm_logger::init(wasm_logger::Config::default());

	log::info!("starting...");
	let game = Game::create(20, 20, 20.)?;
	game.start()?;

	Ok(())
}

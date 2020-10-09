mod game;
use game::game::Game;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	wasm_logger::init(wasm_logger::Config::default());

	log::info!("starting...");
	let game = Game::create(640, 480)?;
	game.start();

	Ok(())
}

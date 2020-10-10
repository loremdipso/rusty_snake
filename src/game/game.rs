use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document};

use super::inner::{self, Inner};

pub struct Game {
	inner: Rc<RefCell<Inner>>,
}

impl Game {
	// creates and initializes a new game. This might fail, so I'm avoiding the "new" convention
	pub fn create(num_cols: u32, num_rows: u32, block_size: f64) -> Result<Game, JsValue> {
		let document = web_sys::window().unwrap().document().unwrap();
		let width = block_size * num_cols as f64;
		let height = block_size * num_rows as f64;
		let (canvas, context) = create_canvas(&document, width, height)?;

		let game = Game {
			inner: Rc::new(RefCell::new(Inner::new(
				num_cols, num_rows, block_size, canvas, context,
			))),
		};

		Game::add_handlers(&game)?;
		game.inner.clone().borrow_mut().focus()?;

		Ok(game)
	}

	pub fn start(&self) -> Result<(), JsValue> {
		let window = web_sys::window().unwrap();
		let game = self.inner.clone();
		let closure = Closure::wrap(Box::new(move || {
			game.borrow_mut()
				.tick()
				.expect("Something's gone wrong with tick");
		}) as Box<dyn FnMut()>);
		window.set_interval_with_callback_and_timeout_and_arguments_0(
			closure.as_ref().unchecked_ref(),
			inner::FPS,
		)?;
		closure.forget();
		Ok(())
	}

	fn add_handlers(&self) -> Result<(), JsValue> {
		{
			let game_copy = self.inner.clone();
			let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
				game_copy
					.borrow_mut()
					.handle_key(event.key())
					.expect("Something's gone wrong with handle key");
			}) as Box<dyn FnMut(_)>);
			self.inner
				.borrow_mut()
				.canvas
				.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
			closure.forget();
		}

		{
			let game_copy = self.inner.clone();
			let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
				log::info!("lost focus");
				game_copy
					.borrow_mut()
					.show_focus_banner()
					.expect("Something's gone wrong with show focus");
			}) as Box<dyn FnMut(_)>);
			self.inner
				.borrow_mut()
				.canvas
				.add_event_listener_with_callback("focusout", closure.as_ref().unchecked_ref())?;
			closure.forget();
		}

		{
			let game_copy = self.inner.clone();
			let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
				game_copy
					.borrow_mut()
					.hide_focus_banner()
					.expect("Something's gone wrong with hide focus");
			}) as Box<dyn FnMut(_)>);
			self.inner
				.borrow_mut()
				.canvas
				.add_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref())?;
			closure.forget();
		}

		Ok(())
	}
}

pub fn create_canvas(
	document: &Document,
	width: f64,
	height: f64,
) -> Result<(web_sys::HtmlCanvasElement, Rc<CanvasRenderingContext2d>), JsValue> {
	let canvas = document
		.get_element_by_id("snake")
		.unwrap()
		.dyn_into::<web_sys::HtmlCanvasElement>()?;
	canvas.style().set_property("background-color", "black")?;
	canvas.style().set_property("margin-left", "auto")?;
	canvas.style().set_property("margin-right", "auto")?;
	canvas.style().set_property("display", "block")?;
	canvas.set_attribute("tabindex", "0")?; // needed for keydown to work
	canvas.set_width(width as u32);
	canvas.set_height(height as u32);

	let context = Rc::new(
		canvas
			.get_context("2d")?
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()?,
	);

	Ok((canvas, context))
}

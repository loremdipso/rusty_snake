use core::cell::RefMut;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlElement};

use super::inner::Inner;

const ANIM_DELAY: i32 = (0.025 * 1000.0) as i32; // 0.025 sec -> 40 fps

pub struct Game {
	inner: Rc<RefCell<Inner>>,
}

impl Game {
	// creates and initializes a new game. Might fail, so I'm avoiding the "new" keyword
	pub fn create(width: f64, height: f64) -> Result<Game, JsValue> {
		let document = web_sys::window().unwrap().document().unwrap();
		let (canvas, context) = create_canvas(&document, &document.body().unwrap())?;

		let game = Game {
			inner: Rc::new(RefCell::new(Inner {
				width: width,
				height: height,
				canvas: canvas,
				context: context,
				should_show_focus_banner: false,
				paused: false,
			})),
		};

		Game::add_handlers(&game)?;
		game.inner.clone().borrow_mut().focus()?;

		Ok(game)
	}

	pub fn start(&self) -> Result<(), JsValue> {
		let window = web_sys::window().unwrap();
		let game = self.inner.clone();
		let closure = Closure::wrap(Box::new(move || {
			game.borrow_mut().update();
		}) as Box<dyn FnMut()>);
		window.set_interval_with_callback_and_timeout_and_arguments_0(
			closure.as_ref().unchecked_ref(),
			ANIM_DELAY,
		)?;
		closure.forget();
		Ok(())
	}

	fn add_handlers(&self) -> Result<(), JsValue> {
		{
			let game_copy = self.inner.clone();
			let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
				game_copy.borrow_mut().handle_key(event.key());
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
				game_copy.borrow_mut().show_focus_banner();
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
				game_copy.borrow_mut().hide_focus_banner();
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
	parent: &HtmlElement,
) -> Result<(web_sys::HtmlCanvasElement, Rc<CanvasRenderingContext2d>), JsValue> {
	let canvas = document
		.create_element("canvas")?
		.dyn_into::<web_sys::HtmlCanvasElement>()?;
	canvas.style().set_property("background-color", "black")?;
	canvas.style().set_property("margin-left", "auto")?;
	canvas.style().set_property("margin-right", "auto")?;
	canvas.style().set_property("display", "block")?;
	canvas.set_attribute("tabindex", "0")?; // needed for keydown to work
	canvas.set_width(640);
	canvas.set_height(480);
	parent.append_child(&canvas)?;

	let context = Rc::new(
		canvas
			.get_context("2d")?
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()?,
	);

	Ok((canvas, context))
}

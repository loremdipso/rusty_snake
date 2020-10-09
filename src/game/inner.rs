use core::cell::RefMut;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlElement};

pub struct Inner {
	pub width: u32,
	pub height: u32,
	pub canvas: web_sys::HtmlCanvasElement,
	pub context: Rc<CanvasRenderingContext2d>,

	pub should_show_focus_banner: bool,
	pub paused: bool,
}

impl Inner {
	pub fn focus(&self) -> Result<(), JsValue> {
		self.canvas.focus()
	}

	pub fn handle_key(&mut self, key: String) -> Result<(), JsValue> {
		log::info!("Received {}", key);
		Ok(())
	}

	pub fn show_focus_banner(&mut self) -> Result<(), JsValue> {
		log::info!("Show focus banner");
		self.should_show_focus_banner = true;
		Ok(())
	}

	pub fn hide_focus_banner(&mut self) -> Result<(), JsValue> {
		log::info!("Hide focus banner");
		self.should_show_focus_banner = false;
		Ok(())
	}

	pub fn update(&mut self) -> Result<(), JsValue> {
		log::info!("updating");
		Ok(())
	}
}

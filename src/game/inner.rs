use core::cell::RefMut;
use std::cell::{Cell, RefCell};
use std::f64;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlElement};

pub struct Inner {
	pub width: f64,
	pub height: f64,
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
		self.process_keys();
		self.draw();
		Ok(())
	}

	pub fn process_keys(&mut self) -> Result<(), JsValue> {
		Ok(())
	}

	pub fn draw(&mut self) -> Result<(), JsValue> {
		let context = &self.context;

		// context.begin_path();

		// context.set_stroke_style(&JsValue::from("white"));

		// // Draw the outer circle.
		// context
		// 	.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
		// 	.unwrap();
		// // Draw the mouth.
		// context.move_to(110.0, 75.0);
		// context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)?;
		// // Draw the left eye.
		// context.move_to(65.0, 65.0);
		// context
		// 	.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
		// 	.unwrap();
		// // Draw the right eye.
		// context.move_to(95.0, 65.0);
		// context
		// 	.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
		// 	.unwrap();
		// context.stroke();

		if self.should_show_focus_banner {
			context.set_stroke_style(&JsValue::from("white"));
			context.set_fill_style(&JsValue::from("white"));
			context.fill_text_with_max_width("hello", 95.0, 65.0, self.width);
		}
		Ok(())
	}
}

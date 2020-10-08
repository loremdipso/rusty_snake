use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	wasm_logger::init(wasm_logger::Config::default());

	log::info!("starting...");
	let document = web_sys::window().unwrap().document().unwrap();
	let (canvas, context) = create_canvas(&document, &document.body().unwrap())?;

	add_handlers(&canvas, &context)?;

	Ok(())
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

pub fn add_handlers(
	canvas: &web_sys::HtmlCanvasElement,
	context: &Rc<CanvasRenderingContext2d>,
) -> Result<(), JsValue> {
	let pressed = Rc::new(Cell::new(false));

	{
		let context = context.clone();
		let pressed = pressed.clone();
		let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
			context.begin_path();
			context.move_to(event.offset_x() as f64, event.offset_y() as f64);
			pressed.set(true);
		}) as Box<dyn FnMut(_)>);
		canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
		closure.forget();
	}

	{
		let context = context.clone();
		let pressed = pressed.clone();
		let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
			if pressed.get() {
				context.set_stroke_style(&JsValue::from("green"));
				context.line_to(event.offset_x() as f64, event.offset_y() as f64);
				context.stroke();
				context.begin_path();
				context.move_to(event.offset_x() as f64, event.offset_y() as f64);
			}
		}) as Box<dyn FnMut(_)>);
		canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
		closure.forget();
	}

	{
		let context = context.clone();
		let pressed = pressed.clone();
		let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
			pressed.set(false);
			context.line_to(event.offset_x() as f64, event.offset_y() as f64);
			context.stroke();
		}) as Box<dyn FnMut(_)>);
		canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
		closure.forget();
	}

	Ok(())
}

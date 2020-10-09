use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::f64;
use std::rc::Rc;
use tau::TAU;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub const FPS: i32 = (0.025 * 1000.0) as i32; // 0.025 sec -> 40 fps
const MIN_SPEED: u32 = 4; // number of frames between updates
const MAX_SPEED: u32 = 3; // number of frames between updates

const NUM_APPLES: usize = 1; // 10 frames between updates

const MAX_KEY_BUFF_LEN: usize = 3; // how many keys we'll keep track of before ignoring inputs
const SNAKE_COLOR: &str = "green"; // how many keys we'll keep track of before ignoring inputs
const APPLE_COLOR: &str = "red"; // how many keys we'll keep track of before ignoring inputs

#[derive(Debug, Clone, Copy)]
struct Vector2D {
	x: i32,
	y: i32,
}

enum ICellContents {
	Empty,
	Snake,
	Apple,
}

pub struct Inner {
	pub width: f64,
	pub height: f64,
	pub canvas: web_sys::HtmlCanvasElement,
	pub context: Rc<CanvasRenderingContext2d>,

	num_squares_x: i32,
	num_squares_y: i32,
	rect_size: f64,

	should_show_focus_banner: bool,
	is_paused: bool,
	is_game_over: bool,
	did_win: bool,
	score: u32,
	key_buff: VecDeque<String>,

	apples: VecDeque<Vector2D>,
	is_growing: bool,

	frames_between_updates: u32,
	frames_until_update: u32,

	head_direction: Vector2D,
	path: VecDeque<Vector2D>,

	rng: ThreadRng,
}

impl Inner {
	pub fn new(
		num_cols: u32,
		num_rows: u32,
		block_size: f64,
		canvas: web_sys::HtmlCanvasElement,
		context: Rc<CanvasRenderingContext2d>,
	) -> Inner {
		let width = block_size * num_cols as f64;
		let height = block_size * num_rows as f64;

		let mut inner = Inner {
			width: width as f64,
			height: height as f64,
			canvas: canvas,
			context: context,

			num_squares_x: num_cols as i32,
			num_squares_y: num_rows as i32,
			rect_size: block_size,

			should_show_focus_banner: false,
			is_paused: false,
			is_game_over: false,
			did_win: false,
			score: 0,
			key_buff: VecDeque::with_capacity(MAX_KEY_BUFF_LEN),

			apples: VecDeque::new(),
			is_growing: false,

			frames_between_updates: MIN_SPEED,
			frames_until_update: 0,

			head_direction: Vector2D { x: 1, y: 0 },
			path: VecDeque::new(),

			rng: rand::thread_rng(),
		};

		if let Some(space) = inner.get_random_empty_space() {
			inner.path.push_front(space);
		}

		return inner;
	}

	fn reset(&mut self) {
		self.is_game_over = false;
		self.did_win = false;
		self.is_growing = false;
		self.path.clear();
		self.path.push_front(Vector2D {
			x: self.num_squares_x / 2,
			y: self.num_squares_y / 2,
		});

		self.score = 0;
		self.apples.clear();
		self.frames_between_updates = MIN_SPEED;
		self.frames_until_update = MIN_SPEED;
	}

	pub fn focus(&self) -> Result<(), JsValue> {
		self.canvas.focus()
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

	pub fn tick(&mut self) -> Result<(), JsValue> {
		self.pre_process_keys();
		if !self.effectively_paused() {
			if self.frames_until_update == 0 {
				self.process_key();
				self.update()?;
				self.frames_until_update = self.frames_between_updates;
			}
			self.frames_until_update -= 1;
		}
		self.draw().expect("Something's gone wrong with draw");
		Ok(())
	}

	fn update(&mut self) -> Result<(), JsValue> {
		let current_head = {
			let head = self.path.back().unwrap(); // this will never be null
			Vector2D {
				x: head.x + self.head_direction.x,
				y: head.y + self.head_direction.y,
			}
		};

		if self.check_game_over(&current_head) {
			self.is_game_over = true;
			if let None = self.get_random_empty_space() {
				self.did_win = true;
			}
			return Ok(());
		}

		// move snake
		if self.is_growing {
			self.is_growing = false;
		} else {
			self.path.pop_front();
		}
		self.path.push_back(current_head);

		// remove apples
		for apple_index in 0..self.apples.len() {
			let apple = self.apples[apple_index];
			if apple.x == current_head.x && apple.y == current_head.y {
				self.apples.swap_remove_back(apple_index);
				self.is_growing = true;
				self.score += 1;
				if self.score % 5 == 0 {
					self.frames_between_updates = max(MAX_SPEED, self.frames_between_updates - 1);
				}
				break;
			}
		}

		// add missing apples
		while self.apples.len() < NUM_APPLES {
			match self.get_random_empty_space() {
				None => {
					break;
				}
				Some(apple) => {
					self.apples.push_back(apple);
				}
			}
		}

		Ok(())
	}

	fn check_game_over(&mut self, new_head: &Vector2D) -> bool {
		if new_head.x < 0
			|| new_head.y < 0
			|| new_head.x > self.num_squares_x
			|| (new_head.x + 1) as f64 * self.rect_size > self.width
			|| (new_head.y + 1) as f64 * self.rect_size > self.height
		{
			return true;
		}

		for pos in self.path.iter() {
			if pos.x == new_head.x && pos.y == new_head.y {
				return true;
			}
		}

		return false;
	}

	pub fn handle_key(&mut self, key: String) -> Result<(), JsValue> {
		log::info!("Received {}", key);
		if self.key_buff.len() < MAX_KEY_BUFF_LEN {
			self.key_buff.push_back(key);
		}
		Ok(())
	}

	fn effectively_paused(&self) -> bool {
		self.should_show_focus_banner || self.is_paused || self.is_game_over
	}

	pub fn pre_process_keys(&mut self) {
		let mut should_reset = false;
		if let Some(key) = self.key_buff.front() {
			match key.as_str() {
				"r" => {
					log::info!("resetting");
					should_reset = true;
					self.key_buff.pop_front();
				}

				"Enter" | " " => {
					if self.is_game_over {
						should_reset = true;
					} else {
						self.is_paused = !self.is_paused;
					}
					self.key_buff.pop_front();
				}
				_ => {}
			}
		}

		// some things we need to do after our immutable borrows up top
		if should_reset {
			self.reset();
		}

		// eats up any keys that would otherwise clog the buffer.
		// Also prevents pause-buffering
		if self.effectively_paused() {
			self.key_buff.clear();
		}
	}

	pub fn process_key(&mut self) {
		if let Some(key) = self.key_buff.pop_front() {
			if self.effectively_paused() {
				return;
			}

			match key.as_str() {
				// NOTE: y is flipped here since that's the default for rendering, and it's easier
				// to flip it just here than anytime we draw
				"ArrowUp" => self.head_direction = Vector2D { y: -1, x: 0 },
				"ArrowDown" => self.head_direction = Vector2D { y: 1, x: 0 },

				"ArrowRight" => self.head_direction = Vector2D { x: 1, y: 0 },
				"ArrowLeft" => self.head_direction = Vector2D { x: -1, y: 0 },
				_ => {}
			}
		}
	}

	pub fn draw(&mut self) -> Result<(), JsValue> {
		let context = &self.context;
		context.clear_rect(0., 0., self.width, self.height);

		self.draw_rects(self.path.iter(), SNAKE_COLOR);
		self.draw_circles(self.apples.iter(), APPLE_COLOR);

		if self.is_paused {
			self.draw_banner("PAUSED");
		} else if self.is_game_over {
			if self.did_win {
				self.draw_banner("YOU WON!!!");
			} else {
				self.draw_banner("GAME OVER");
			}
		} else if self.should_show_focus_banner {
			self.draw_banner("LOST FOCUS");
		}
		Ok(())
	}

	fn draw_rects<'a, I>(&self, rects: I, color: &str)
	where
		I: Iterator<Item = &'a Vector2D>,
	{
		let context = &self.context;
		context.save();
		context.set_fill_style(&JsValue::from(color));
		context.set_stroke_style(&JsValue::from("black"));
		context.set_line_width(1.);
		for pos in rects {
			context.begin_path();
			context.rect(
				self.rect_size * pos.x as f64,
				self.rect_size * pos.y as f64,
				self.rect_size,
				self.rect_size,
			);
			context.fill();
			context.stroke();
		}
		context.restore();
	}

	fn draw_circles<'a, I>(&self, circles: I, color: &str)
	where
		I: Iterator<Item = &'a Vector2D>,
	{
		let context = &self.context;
		let radius = self.rect_size / 2.;
		let border = 2.;
		context.save();
		context.set_fill_style(&JsValue::from(color));
		context.set_stroke_style(&JsValue::from("black"));
		context.set_line_width(1.);
		for pos in circles {
			context.begin_path();
			context
				.arc(
					self.rect_size * pos.x as f64 + radius,
					self.rect_size * pos.y as f64 + radius,
					radius - border,
					0.,
					TAU,
				)
				.unwrap();
			context.fill();
			context.stroke();
		}
		context.restore();
	}

	fn draw_banner(&self, text: &str) {
		let context = &self.context;
		context.save();
		context.set_fill_style(&JsValue::from("white"));
		context.set_global_alpha(0.5);
		let quarter_height = self.height / 4.;
		context.fill_rect(
			0.,
			quarter_height,
			self.width,
			self.height - quarter_height * 2.,
		);
		context.restore();

		context.save();
		context.begin_path();
		context.set_font("60px Arial");
		context.set_stroke_style(&JsValue::from("white"));
		context.set_font("60px Arial");
		context.set_text_align("center");
		context.set_text_baseline("middle");
		context.set_fill_style(&JsValue::from("white"));
		context
			.fill_text_with_max_width(text, self.width / 2., self.height / 2., self.width)
			.expect("Something's gone wrong here");
		context.restore();
	}

	fn get_random_empty_space(&mut self) -> Option<Vector2D> {
		let empty_squares = self.get_empty_squares();
		if let Some(space) = empty_squares.choose(&mut self.rng) {
			return Some(Vector2D {
				x: space.x,
				y: space.y,
			});
		}
		return None;
	}

	fn get_empty_squares(&mut self) -> Vec<Vector2D> {
		let mut rv = vec![];
		for x in 0..self.num_squares_x {
			for y in 0..self.num_squares_y {
				if let ICellContents::Empty = self.contents_of_square(x, y) {
					rv.push(Vector2D { x: x, y: y });
				}
			}
		}
		return rv;
	}

	fn contents_of_square(&self, x: i32, y: i32) -> ICellContents {
		for pos in self.path.iter() {
			if pos.x == x && pos.y == y {
				return ICellContents::Snake;
			}
		}

		for pos in self.apples.iter() {
			if pos.x == x && pos.y == y {
				return ICellContents::Apple;
			}
		}

		return ICellContents::Empty;
	}
}
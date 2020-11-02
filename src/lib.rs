use simple_logger::SimpleLogger;
use log::*;
extern crate glfw;

use glfw::{Context};

use image::imageops::{resize, Nearest};
use image::{open as open_image, DynamicImage};
// Statics
static mut IS_OPEN: bool = false;
static mut HAND_CURSOR: bool = false;
static mut CROSSHAIR_CURSOR: bool = false;
static mut DEFAULT_CURSOR: bool = false;
pub struct Window {
	pub title: String,
	pub height: u32,
	pub width: u32,
	pub name: String
}
impl Window {
	pub fn get_title(&self) -> String{
		return self.title.to_string()
	}
	pub fn get_width(&self) -> u32 {
		return self.width
	}
	pub fn get_height(&self) -> u32 {
		return self.height;
	}
	pub fn is_open(&self) -> bool{
		unsafe {
			if !IS_OPEN {
				return false
			}else {
				return true
			}
		}
	}
	pub fn set_cursor_type(&self, cursor: &str){
		unsafe {
			if cursor == "hand" {
				HAND_CURSOR = true;
			}else if cursor == "default" {
				DEFAULT_CURSOR = true;
			}else if cursor == "crosshair" {
				CROSSHAIR_CURSOR = true;
			}
		}
	}
	pub fn start_window(&self, load_func: fn(String) -> String){
		println!("Starting SimpleLogger...");
	    SimpleLogger::new().init().unwrap();
    	info!("Creating Window...");
    	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
		let (mut window, _events) = glfw.create_window(self.get_width(), self.get_height(), &self.get_title().to_string(), glfw::WindowMode::Windowed)
		     .expect("Failed to create GLFW window.");
		window.set_key_polling(true);
		window.make_current();
		info!("Window Created!");
		info!("Checking for cursor type...");
		unsafe {
			if CROSSHAIR_CURSOR {
				info!("Setting Cursor type to crosshair...");
				let crosshair_cursor = glfw::Cursor::standard(glfw::StandardCursor::Crosshair);
				window.set_cursor(Some(crosshair_cursor));
			}else if HAND_CURSOR {
				info!("Setting Cursor type to hand...");
				let hand_cursor = glfw::Cursor::standard(glfw::StandardCursor::Hand);
				window.set_cursor(Some(hand_cursor));
			}else if DEFAULT_CURSOR{
				info!("Setting Cursor type to default...");
				let arrow_cursor = glfw::Cursor::standard(glfw::StandardCursor::Arrow);
				window.set_cursor(Some(arrow_cursor));
			}
		}
		info!("Running load_func code on other thread...");
		std::thread::spawn(move||{
			load_func("Start".to_string());
		});

		unsafe {
			IS_OPEN = true;
		}
		while !window.should_close() {
		    glfw.poll_events();
 	  }
  }
}

use simple_logger::SimpleLogger;
use log::*;
extern crate glfw;

use glfw::{Context};
static mut IS_OPEN: bool = false;
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
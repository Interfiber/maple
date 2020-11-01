use simple_logger::SimpleLogger;
use std::thread;
use log::*;
extern crate glfw;

use glfw::{Action, Context, Key};

pub struct Window {
	pub title: String,
	pub height: i32,
	pub width: i32,
	pub name: String
}
impl Window {
	pub fn get_title(&self) -> String{
		return self.title.to_string()
	}
	pub fn start_window(&self){
		println!("Starting SimpleLogger...");
	    SimpleLogger::new().init().unwrap();
    	info!("Creating Window...");
    	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
		let (mut window, events) = glfw.create_window(300, 300, &self.get_title().to_string(), glfw::WindowMode::Windowed)
		     .expect("Failed to create GLFW window.");
		window.set_key_polling(true);
		window.make_current();
		info!("Window Created!");
		while !window.should_close() {
		    glfw.poll_events();	   
 	  }
  }
}
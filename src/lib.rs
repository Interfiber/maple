use simple_logger::SimpleLogger;
use std::io::Read;
use std::fs::File;
use log::*;


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
	pub fn set_cursor_image(&self, image: &str){
		let mut img_file = File::open(image).unwrap();
		let mut img_data = Vec::new();
		match img_file.read_to_end(&mut img_data){
			Ok(_) => println!("Read data from {}!", image),
			Err(err) => println!("{}", err)
		}
		match std::fs::write("maple-cursor-image.png", img_data){
			Ok(_) => println!("Saved Cursor image tomaple-cursor-image.png"),
			Err(err) => println!("{}! Failed to save cursor image to maple-cursor-image.png!", err)
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
		if std::path::Path::new("maple-cursor-image.png").exists(){
			if let DynamicImage::ImageRgba8(icon) = open_image("maple-cursor-image.png").unwrap() {
        		//Resize icon while preserving aspect ratio
        		let resized_icon = resize(&icon, 32, icon.height() / icon.width() * 32, Nearest);

        		let cursor = glfw::Cursor::create(resized_icon, 0, 0);

				window.set_cursor(Some(cursor));
			}
    	}
		info!("Running load_func code on other thread...");
		std::thread::spawn(move||{
			// Load functions into thread
			pub fn open() -> bool {
				unsafe {
					return IS_OPEN;
				}
			}
			load_func("Start".to_string());
		});

		unsafe {
			IS_OPEN = true;
		}
		while !window.should_close() {
		    glfw.poll_events();
 	  }
  }
  pub fn cleanup(&self){
	  info!("Removing Caches...");
	  if std::path::Path::new("maple-cursor-image.png").exists(){
		  match std::fs::remove_file("maple-cursor-image.png"){
			  Ok(_) => println!("Removed Cursor icon cache!"),
			  Err(err) => println!("Error: {}! Failed to remove cursor icon cache!", err)
		  }
	  }
  }
}

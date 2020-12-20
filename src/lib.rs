// Import Winit Stuff
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use winit::event::{Event, WindowEvent};
// Import std stuff
use std::io::Read;
pub struct OSWindow {
	pub title: String,
	pub x: u32,
	pub y: u32,
	pub log: bool
}
impl OSWindow {
	/// Set resizable makes the window resizable or not resizable by the user. By passing in true It will make it
	/// Resizable. Passing in false will do the opposite.
	/// Example:
	/// ```
	/// use maple::*;
	/// fn main(){
	/// 	let win = OSWindow {
	/// 		title: String::from("My Epic Amazing Window!"),
	///         x: 400,
	///         y: 400,
	///         log: true
	///     };
	///   win.set_resizable(false);
	///   win.create_window();
	/// }
	/// ```
	pub fn set_resizable(&self, resizable: bool) {
			if self.log {
				println!("[INF]:: Creating /tmp/maple_window_resize");
			}
			match std::fs::write("/tmp/maple_window_resize", resizable.to_string()) {
				Ok(_) => print!(""),
				Err(err) => println!("[ERR]:: Error! Failed to crate /tmp/maple_window_resize! Error:\n{}", err)
			}
	}
	/// set_transparent takes in a bool(true/false) which if true will make the window create transparent.
	/// Example:
	/// ```
	/// use maple::*;
	/// fn main(){
	/// 	let win = OSWindow {
	/// 		title: String::from("My Epic Amazing Window!"),
	///         x: 400,
	///         y: 400,
	///         log: true
	///     };
	///   win.set_transparent(true);
	///   win.create_window();
	/// }
	/// ```
	///
	pub fn set_transparent(&self, transparent: bool){
		if self.log {
			println!("[INF]:: Making Window Transparent...");
		}
		if transparent {
			match std::fs::write("/tmp/maple_transparent_window", transparent.to_string()) {
				Ok(_) => print!(""),
				Err(err) => println!("[ERR]:: Failed to create /tmp/maple_transparent_window! Error:\n{}", err)
			}
			if self.log {
				println!("[INF]:: Created /tmp/maple_transparent_window!");
			}
		}
	}
	/// The Set Cursor function is called on the OSWindow. set_cursor()  takes in one argument as a string
	/// Which is the type of the cursor. When the window is loaded that cursor will be the one shown on screen
	/// Example:
	/// ```
	/// use maple::*;
	/// fn main(){
	/// 	let win = OSWindow {
	/// 		title: String::from("My Epic Window with a diffrent cursor"),
	/// 		x: 400,
	/// 		y: 400,
	/// 		log: false
	/// 	};
	/// win.set_cursor("CrossHair");
	/// win.create_window();
	/// }
	/// ```
	///
	pub fn set_cursor(&self, cursor_type: &str){
		if self.log {
			println!("[INF]:: Setting Cursor type...");
		}
		match std::fs::write("/tmp/maple_cursor_type", cursor_type){
			Ok(_) => print!(""),
			Err(err) => println!("[ERR]:: Failed to write to /tmp/maple_cursor_type! Error: \n{}", err)
		}
		if self.log {
			println!("[INF]:: Cursor Setup Complete!");
		}
	}
	/// The Create Window function is called on the OSWindow. create_window() will spawn a GUI Window
	/// On the users screen with the specified settings delared before the window was launched. Such as color
	/// Example:
	/// ```
	/// use maple::*;
	/// fn main(){
	/// 	let win = OSWindow {
	///			title: String::from("My Epic Amazing Window!"),
	///         x: 400,
	///         y: 400,
	///         log: true
	///		};
	///   win.create_window();
	/// }
	/// ```
	pub fn create_window(&self){
		// Create Winit Stuff
		if self.log {
			println!("[INF]:: Creating Event Loop")
		}
		let event_loop = EventLoop::new();
		if self.log {
			println!("[INF]:: Creating Window Builder...");
		}
		let mut transparent = false;
		if std::path::Path::new("/tmp/maple_transparent_window").exists(){
			if self.log {
				println!("[INF]:: Making Window Transparent...");
			}
			transparent = true;
			if self.log {
				println!("[INF]:: Removing file /tmp/maple_transparent_windows...");
			}
			match std::fs::remove_file("/tmp/maple_transparent_window"){
				Ok(_) => print!(""),
				Err(err) => println!("[ERR]:: Failed to delete /tmp/maple_transparent_window! Error:\n{}", err)
			}
		}
		let window = WindowBuilder::new().with_transparent(transparent).build(&event_loop).unwrap();
		if self.log {
			println!("[INF]:: Setting Window Size...");
		}
		// Set Window Min Size
		window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(self.x, self.y)));
		// Set Window Max Size
		window.set_max_inner_size(Some(winit::dpi::LogicalSize::new(self.x, self.y)));
		if self.log {
			println!("[INF]:: Setting Window Title...");
		}
		window.set_title(&self.title);
		if std::path::Path::new("/tmp/maple_window_resize").exists(){
			if self.log {
				println!("[INF]:: Making Window resizable or not resizable...");
			}
			let mut resize_tmp_file = std::fs::File::open("/tmp/maple_window_resize").unwrap();
			let mut resize = String::from("");
			match resize_tmp_file.read_to_string(&mut resize){
				Ok(_) => print!(""),
				Err(err) => println!("[ERR]:: Failed to read from /tmp/maple_window_resize! Error:\n{}", err)
			}
			// Make window resizable
			let resizeable = resize.parse::<bool>().unwrap();
			if self.log {
				if resizeable {
					println!("[INF]:: Making Window resizable by user...");
				}else {
					println!("[INF]:: Making Window not resizable by user...");
				}
			}
			window.set_resizable(resizeable);
			if self.log {
				println!("[INF]:: Deleting /tmp/maple_window_resize...");
				match std::fs::remove_file("/tmp/maple_window_resize"){
					Ok(_) => print!(""),
					Err(err) => println!("[ERR]:: Failed to remove /tmp/maple_window_resize! Error: \n{}", err)
				}
			}
		}
		if std::path::Path::new("/tmp/maple_cursor_type").exists(){
			if self.log {
				println!("[INF]:: Setting Cursor Type...");
			}
			let mut cursor_file = std::fs::File::open("/tmp/maple_cursor_type").unwrap();
			let mut cursor = String::from("");
			match cursor_file.read_to_string(&mut cursor){
				Ok(_) => print!(""),
				Err(err) => println!("[ERR]:: Failed to read from /tmp/maple_cursor_type! Error: \n {}", err)
			}
			if self.log {
				println!("[INF]:: Trying to set cursor type to {}...", cursor);
			}
			// Detect What cursor it is
			if cursor.contains("CrossHair"){
				// Crosshair
				window.set_cursor_icon(winit::window::CursorIcon::Crosshair);
			}else if cursor.contains("Hand"){
				// Hand
				window.set_cursor_icon(winit::window::CursorIcon::Hand);
			}else if cursor.contains("Default") || cursor.contains("Arrow"){
				// Arrow Or Default
				window.set_cursor_icon(winit::window::CursorIcon::Arrow);
			}else if cursor.contains("QuestionMark"){
				// What?
				window.set_cursor_icon(winit::window::CursorIcon::Help);
			}else if cursor.contains("Waiting") ||cursor.contains("BeachBall") {
				// Waiting
				window.set_cursor_icon(winit::window::CursorIcon::Wait);
			}else if cursor.contains("Text"){
				// Text Input
				window.set_cursor_icon(winit::window::CursorIcon::Text);
			}else if cursor.contains("MagnifyIn"){
				// Magnify In
				window.set_cursor_icon(winit::window::CursorIcon::ZoomIn);
			}else if cursor.contains("MagnifyOut"){
				// Magnify Out
				window.set_cursor_icon(winit::window::CursorIcon::ZoomOut);
			}else {
				if self.log {
					println!("[WAR]:: No Such Cursor Type registered as {}! Using Default Cursor Type", cursor);
				}
				window.set_cursor_icon(winit::window::CursorIcon::Arrow);
			}
			match std::fs::remove_file("/tmp/maple_cursor_type"){
				Ok(_) => print!(""),
				Err(err) => println!("[ERR]:: Failed to delete /tmp/maple_cursor_image! Error: \n{}", err)
			}
		}
		if self.log {
			println!("[INF]:: Running Event Loop...");
		}
		event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;
			match event {
				Event::WindowEvent {
					event: WindowEvent::CloseRequested,
					window_id,
				} if window_id == window.id() => *control_flow = ControlFlow::Exit,
				_ => (),
			}
		});
	}
}

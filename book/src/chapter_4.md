# Chapter 4: Fullscreen windows
If you want a window that is fullscreen in maple its really easy. You can use the following code to make a fullscreen window
```rust
use maple::*;
fn after_load(place_holder: String) -> String{
	return place_holder;
}
fn main(){
	let win = Window {
		title: String::from("My Awsome Window!"),
		height: 400,
		width: 400,
		name: String::from("Maple Test")
	};
	// Set window to fullscreen
	win.set_fullscreen();
	// Create the window on the screen
	win.start_window(after_load);
}
```
"win.set_fullscreen()" makes the window fullscreen

use maple::*;
// This function will be ran when the window loads
fn after_load(place_holder: String) -> String{
	return place_holder;
}
fn main(){
	// Create Window object that is named win.
	let win = Window {
		title: String::from("My Awsome Window!"),
		height: 400,
		width: 400,
		name: String::from("Maple Test")
	};
	// Set cursor type
	win.set_cursor_type("crosshair");
	// Create the window on the screen
	win.start_window(after_load);
}
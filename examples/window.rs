use maple::*;

fn main(){
	let win = Window {
		title: String::from("My Awsome Window!"),
		height: 14,
		width: 14,
		name: String::from("Maple Test")
	};
	win.start_window();
}
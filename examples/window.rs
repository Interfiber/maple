use maple::*;

fn main(){
	let win = Window {
		title: String::from("My Awsome Window!"),
		height: 900,
		width: 900,
		name: String::from("Maple Test")
	};
	win.start_window();
}
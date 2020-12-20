use maple::*;
fn main(){
	let win = OSWindow {
		title: String::from("My Epic Amazing Window!"),
        x: 400,
        y: 400,
        log: false
	};
  win.create_window();
}
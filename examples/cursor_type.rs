use maple::*;
fn main(){
	let win = OSWindow {
		title: String::from("My Epic Window with a diffrent cursor"),
        x: 400,
        y: 400,
        log: true
	};
  win.set_cursor("Waiting");
  win.create_window();
}
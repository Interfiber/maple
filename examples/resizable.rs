use maple::*;
fn main(){
    let win = OSWindow {
        title: String::from("My Epic Amazing Window!"),
        x: 400,
        y: 400,
        log: true
    };
    win.set_resizable(false);
    win.create_window();
}
# Chapter 3: Cursors Oh My!

If you want your window to have a different cursor type like a finger cursor supplied my the OS. Or a custom cursor from an image maple has support for that.


## Cursor Types

Below is example code for using cursor types.
```rust
use maple::*;
// This function will be ran when the window loads
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
	win.set_cursor_type("crosshair");
	win.start_window(after_load);
}
```
This will set the cursor type to the crosshair cursor. But heres a full list of supported cursors

  - "hand"
  - "crosshair"
  - "default"
If you provide an incorrect cursor name you will get the default cursor for your OS which is the arrow. The line that has "win.set_cursor_type" on it it the line that sets the windows cursor.

## Custom Cursors
The default cursors are fun and all but how about one that comes from an images. The code below will change the windows cursor to become an image
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
	win.set_cursor_image("cursor.png");
	win.start_window(after_load);
	win.cleanup();
}
```
This code will set the cursor to the image from "cursor.png". You will also see the cleanup function. This will remove some cursor caches generated at runtime. In this case it will delete a file called ```maple-cursor-image.png```

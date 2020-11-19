# Chapter 1: Getting Started

Maple is an easy to use Lightweight GUI Library written in rust that sits on top of the glfw crate. To install it we will need to add it to our Cargo.toml. To do that place the following under dependencies.
```toml
maple = { git = "https://github.com/Interfiber/maple" }
```
Now to compile maple into our project run
```
cargo build
```
this will compile your project with maple. Now that its compiled we can create our first window in the next chapter.

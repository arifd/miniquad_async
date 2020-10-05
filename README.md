# miniquad_async

This template for a Rust project brings async/.await syntax into your [Miniquad](https://github.com/not-fl3/Miniquad) project without bringing in a runtime! (Your async code is blocking, however!)

This allows you to synchronously preload items (such as loading assets) while inside the Miniquad execution thread, or simply to execute any code once before the main draw loop.

I have provided an `async fn load_file("path/to/file")` which wraps Miniquad's callback based function into a `Future` to avoid [Callback Hell](http://callbackhell.com/)

Any drawing should take place in `fn run()` with Context living as a global `static mut`. Dangerous, yes, but necessary.

Most of the relevant parts of the work all come straight out of [Macroquad](https://github.com/not-fl3/macroquad), which itself is an abstraction over Miniquad solving the exact same problem + more. If you are looking for a more feature rich abstraction, use Macroquad.
# 🦀 rust-test

My small repo for learning rust. There's no real purpose to this repo other than for me to learn rust and document my progress.  
  
❗ Quick disclaimer: My code is obviously not the best and you shouldn't fully rely on it.  
If you need real help, please check out [The Rust Programming Language book](https://doc.rust-lang.org/book/).

## Examples

- [Fibonacci Calculation](./src/examples/fibonacci.rs)
- [Celsius to Fahrenheit Conversion](./src/examples/celsius.rs)
- [Ownership and borrowing](./src/examples/ownership.rs)
- [Rectangle/square examples using structs](./src/examples/rectangle.rs)
- [User register logic using structs](./src/examples/user.rs)
- [IP object building with enums and structs](./src/examples/ip.rs)
- [Requesting data from an example API](./src/examples/request_test.rs)

## Usage

```bash
cargo run --example <option>
```
Check [src/main.rs](./src/main.rs) or don't provide any option to see all `--example` options.  
Some examples contain additional arguments. Usually you can check them out with
```bash
cargo run --example <option> --help
```
## Building

There are 2 ways to build the project that will be ready for release:

```bash
cargo build --release # builds the project in release mode (optimized)
```
...and
```bash
./build.sh # builds the project in release mode (optimized + applied additional optimization flags.)
```
Required for the second option to work:
- nightly Rustup and Rust-src toolchain
  ```bash
  rustup toolchain install nightly
  rustup component add rust-src --toolchain nightly
  ```
- `upx` package installed for the binary to be compressed on host machine
  ```bash
  sudo apt install upx-ucl # Use appropriate package manager for your OS.
  ```

Second option is recommended when possible - for getting the smallest binary size. Experience from using the file shouldn't be affected in normal cases. (5x smaller file size)

extern crate image_base64_wasm;

fn main() {
    let base64 = "base64 String";
    let image = image_base64_wasm::from_base64(base64);

    let image_path = "/Users/yuvan/Documents/github/hellowasm/target/wasm32-unknown-unknown/release/hellowasm.wasm";
    let base64 = image_base64_wasm::to_base64(image_path);
    print!(base64);
}
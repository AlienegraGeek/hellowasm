#[no_mangle]
pub extern fn fbin(x: i32) -> i32 {
    return if x <= 1 {
        1
    } else {
        // return fbin(x - 1) + fbin(x - 2);
        x * x
    };
}


extern crate image_base64_wasm;
extern crate base64;

use base64::*;

#[test]
fn trans_image() {
    // let base64 = "base64 String";
    // let image = image_base64_wasm::from_base64(base64);

    // let image_path = "/Users/yuvan/Documents/github/hellowasm/target/wasm32-unknown-unknown/release/hellowasm.wasm";
    let image_path = "/Users/yuvan/Documents/github/hellowasm/target/wasm32-unknown-unknown/release/test.jpeg";
    let base64 = image_base64_wasm::to_base64(image_path);
    println!("{}", base64);
}

#[test]
fn trans_binary() {
    let wat = wasmprinter::print_file("/Users/yuvan/Documents/github/hellowasm/target/wasm32-unknown-unknown/release/hellowasm.wasm");
    // print!("{}", wat.unwrap());
    let wat_str = wat.unwrap();
    println!("{}", wat_str);
    let encoded = encode(wat_str);
    println!("Base64: {}", encoded);
    // let binary = /* ... */;
    // let wat = wasmprinter::print_bytes(&binary)?;
}
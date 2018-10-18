#![feature(proc_macro)]
extern crate wasm_bindgen;

//use std::io::prelude::*;
//use std::net::TcpStream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    type console;

    #[wasm_bindgen(static = console)]
    fn log(s: &str);

}

//#[no_mangle]
#[wasm_bindgen]
pub fn hello_world() -> i8 {
    //match TcpStream::connect("127.0.0.1:34254") {
        //Ok(stream) => {}
        //Err(error_message) => {
			//let ptr = CString::new("hello there").unwrap().as_ptr();
			//unsafe {
                 //ffi::log(ptr); 
			//}
		//}
    //}

    // ignore the Result
    //let _ = stream.write(&[1]);
    //let _ = stream.read(&mut [0; 128]); // ignore here too
    console::log("Hello, there!");
    "hello world".as_ptr() as i8
}

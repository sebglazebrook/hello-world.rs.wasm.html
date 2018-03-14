#[no_mangle]
pub fn hello_world() -> i8 {
    "hello world".as_ptr() as i8
}

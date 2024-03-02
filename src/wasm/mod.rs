pub mod draw;

mod sys {
    #[link(wasm_import_module = "dito2")]
    extern "C" {
        pub fn log(ptr: *const u8, len: usize);
        pub fn error(ptr: *const u8, len: usize);
        pub fn fill_rect(x: isize, y: isize, w: usize, h: usize, r: u8, g: u8, b: u8);
    }
}

pub fn log(msg: &str) {
    unsafe { sys::log(msg.as_ptr(), msg.len()) }
}

pub fn error(msg: &str) {
    unsafe { sys::error(msg.as_ptr(), msg.len()) }
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        $crate::wasm::log(&format!($($t)*))
    };
}

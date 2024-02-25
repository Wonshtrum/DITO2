use crate::wasm::draw::Rectangle;

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

pub fn fill_rect(rect: Rectangle) {
    unsafe {
        sys::fill_rect(
            rect.bbox.x,
            rect.bbox.y,
            rect.bbox.w,
            rect.bbox.h,
            rect.fill.r,
            rect.fill.g,
            rect.fill.b,
        )
    }
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        $crate::wasm::log(&format!($($t)*))
    };
}
pub mod draw;

mod sys {
    #[link(wasm_import_module = "dito2")]
    extern "C" {
        pub fn log(ptr: *const u8, len: usize);
        pub fn error(ptr: *const u8, len: usize);
        pub fn draw_quad(
            x: isize,
            y: isize,
            w: usize,
            h: usize,
            tex: usize,
            r: u8,
            g: u8,
            b: u8,
            a: u8,
        );
        pub fn new_mesh(ptr: *const u8, len: usize) -> usize;
        pub fn update_mesh(id: usize, ptr: *const u8, len: usize);
        pub fn free_mesh(id: usize);
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

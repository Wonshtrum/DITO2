use crate::wasm::sys;

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const BLACK: Self = Self::hex(0x000000);
    pub const GREY: Self = Self::hex(0xAAAAAA);
    pub const WHITE: Self = Self::hex(0xFFFFFF);
    pub const RED: Self = Self::hex(0xFF0000);
    pub const GREEN: Self = Self::hex(0x00FF00);
    pub const BLUE: Self = Self::hex(0x0000FF);
    pub const YELLOW: Self = Self::hex(0xFFFF00);
    pub const MAGENTA: Self = Self::hex(0xFF00FF);
    pub const CYAN: Self = Self::hex(0x00FFFF);

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn hex(code: u32) -> Self {
        Self {
            r: (code >> 16) as u8,
            g: (code >> 8) as u8,
            b: (code >> 0) as u8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: isize,
    pub y: isize,
    pub w: usize,
    pub h: usize,
    pub fill: RGB,
}

impl Rectangle {
    pub fn new(x: isize, y: isize, w: usize, h: usize, fill: RGB) -> Self {
        Self { x, y, w, h, fill }
    }
    pub fn square(x: isize, y: isize, s: usize, fill: RGB) -> Self {
        Self {
            x,
            y,
            w: s,
            h: s,
            fill,
        }
    }
}

pub fn fill_rect(rect: Rectangle) {
    unsafe {
        sys::fill_rect(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            rect.fill.r,
            rect.fill.g,
            rect.fill.b,
        )
    }
}

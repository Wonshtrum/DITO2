use core::mem::size_of_val;

use crate::wasm::sys;

#[derive(Debug)]
pub struct MeshRef {
    pub id: usize,
    pub dirty: bool,
}

impl MeshRef {
    pub fn new() -> Self {
        Self { id: 0, dirty: true }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(dead_code)]
impl RGBA {
    pub const BLACK: Self = Self::hex(0x000000FF);
    pub const GREY: Self = Self::hex(0xAAAAAAFF);
    pub const WHITE: Self = Self::hex(0xFFFFFFFF);
    pub const RED: Self = Self::hex(0xFF0000FF);
    pub const GREEN: Self = Self::hex(0x00FF00FF);
    pub const BLUE: Self = Self::hex(0x0000FFFF);
    pub const YELLOW: Self = Self::hex(0xFFFF00FF);
    pub const MAGENTA: Self = Self::hex(0xFF00FFFF);
    pub const CYAN: Self = Self::hex(0x00FFFFFF);

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const fn hex(code: u32) -> Self {
        Self {
            r: (code >> 24) as u8,
            g: (code >> 16) as u8,
            b: (code >> 8) as u8,
            a: (code >> 0) as u8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: isize,
    pub y: isize,
    pub w: usize,
    pub h: usize,
    pub tex: usize,
    pub fill: RGBA,
}

impl Rectangle {
    pub fn new(x: isize, y: isize, w: usize, h: usize, tex: usize, fill: RGBA) -> Self {
        Self {
            x,
            y,
            w,
            h,
            tex,
            fill,
        }
    }
}

pub fn draw_quad(rect: Rectangle) {
    unsafe {
        sys::draw_quad(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            rect.tex,
            rect.fill.r,
            rect.fill.g,
            rect.fill.b,
            rect.fill.a,
        )
    }
}

pub fn new_mesh<T>(vertex_buffer: &[T]) -> MeshRef {
    unsafe {
        let id = sys::new_mesh(vertex_buffer.as_ptr() as _, size_of_val(vertex_buffer));
        MeshRef { id, dirty: false }
    }
}

pub fn update_mesh<T>(mesh: &mut MeshRef, vertex_buffer: &[T]) {
    mesh.dirty = false;
    unsafe {
        sys::update_mesh(
            mesh.id,
            vertex_buffer.as_ptr() as _,
            size_of_val(vertex_buffer),
        )
    }
}

pub fn free_mesh(mesh: &MeshRef) {
    unsafe { sys::free_mesh(mesh.id) }
}

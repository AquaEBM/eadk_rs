use core::{cell::Cell, ffi::c_char, mem::MaybeUninit, ops::{Add, AddAssign}};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
#[repr(C)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Add<[i16; 2]> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, [dx, dy]: [i16; 2]) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x + dx,
            y: y + dy,
        }
    }
}

impl AddAssign<[i16; 2]> for Point {
    #[inline]
    fn add_assign(&mut self, [dx, dy]: [i16; 2]) {
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
#[repr(C)]
pub struct Rect {
    pub point: Point,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    #[inline]
    #[must_use]
    pub fn surface_area(&self) -> u32 {
        self.w as u32 * self.h as u32
    }

    #[inline]
    #[must_use]
    pub fn signed(mut point: Point, size: [i16; 2]) -> Self {
        point += size.map(|coord| coord.min(0));
        let [w, h] = size.map(i16::unsigned_abs);
        Self { point, w, h }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
#[repr(C)]
pub struct Color(pub u16);

impl Color {
    #[inline]
    #[must_use]
    pub const fn from_rgb([r, g, b]: [u8; 3]) -> Self {
        let r = u16::from_be_bytes([r & 0b11111000, 0]);
        let g = ((g & 0b11111100) as u16) << 3;
        let b = (b >> 3) as u16;
        Self(r | g | b)
    }
}

extern "C" {
    fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    fn eadk_display_push_rect(rect: Rect, color: *const Color);
    fn eadk_display_wait_for_vblank() -> bool;
    fn eadk_display_pull_rect(rect: Rect, pixels: *mut Color);
    fn eadk_display_draw_string(text: *const u8, point: Point, big: bool, col: Color, bg: Color);
}

#[inline]
pub unsafe fn draw_string_unchecked(text: *const u8, point: Point, big: bool, col: Color, bg: Color) {
    unsafe { eadk_display_draw_string(text, point, big, col, bg) }
}

#[inline]
#[must_use]
pub fn try_pull_rect(rect: Rect, pixels: &mut [Color]) -> bool {
    if pixels.len() > rect.surface_area() as usize {
        return false;
    }

    unsafe {
        eadk_display_pull_rect(rect, pixels.as_mut_ptr());
    }

    true
}

#[inline]
#[must_use]
pub fn try_pull_rect_cell(rect: Rect, pixels: &[Cell<Color>]) -> bool {
    if pixels.len() > rect.surface_area() as usize {
        return false;
    }

    unsafe {
        eadk_display_pull_rect(rect, pixels.as_ptr().cast_mut().cast());
    }

    true
}

#[inline]
pub fn pull_rect(rect: Rect, pixels: &mut [Color]) {
    assert!(
        try_pull_rect(rect, pixels),
        "pixels slice len ({}) must be at least w x h ({} x {} = {})",
        pixels.len(),
        rect.w,
        rect.h,
        rect.surface_area(),
    )
}

#[inline]
pub fn pull_rect_cell(rect: Rect, pixels: &[Cell<Color>]) {
    assert!(
        try_pull_rect_cell(rect, pixels),
        "pixels slice len ({}) must be at least w x h ({} x {} = {})",
        pixels.len(),
        rect.w,
        rect.h,
        rect.surface_area(),
    )
}

#[inline]
#[must_use]
pub fn get_pixel(point: Point) -> Color {
    let mut col = MaybeUninit::uninit();
    unsafe {
        eadk_display_pull_rect(Rect { point, w: 1, h: 1 }, col.as_mut_ptr());
        col.assume_init()
    }
}

#[inline]
#[must_use]
pub fn try_draw_rect(rect: Rect, pixels: &[Color]) -> bool {
    if pixels.len() > rect.surface_area() as usize {
        return false;
    }

    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }

    true
}

#[inline]
pub fn draw_rect(rect: Rect, pixels: &[Color]) {
    assert!(
        try_draw_rect(rect, pixels),
        "pixels slice len ({}) must be at least w x h ({} x {} = {})",
        pixels.len(),
        rect.w,
        rect.h,
        rect.surface_area(),
    );
}

#[inline]
pub fn fill_rect(rect: Rect, color: Color) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

#[inline]
pub fn set_pixel(point: Point, col: Color) {
    fill_rect(Rect { point, w: 1, h: 1 }, col);
}

#[inline]
pub fn wait_for_vblank() -> bool {
    unsafe { eadk_display_wait_for_vblank() }
}

pub fn draw_line(mut start: Point, end: Point, col: Color) {
    let mut dx = end.x - start.x;
    let sx = dx.signum();
    dx = dx.abs();

    let mut dy = end.y - start.y;
    let sy = dy.signum();
    dy = -dy.abs();

    let mut e = dx + dy;

    loop {
        set_pixel(start, col);

        if start.x == end.x && start.y == end.y {
            break;
        }

        let e2 = e << 1;

        if e2 >= dy {
            if start.x == end.x {
                break;
            }

            e += dy;
            start.x += sx;
        }

        if e2 <= dx {
            if start.y == end.y {
                break;
            }

            e += dx;
            start.y += sy;
        }
    }
}

pub fn draw_circle(center: Point, r: u16, col: Color) {
    let mut sx = r as i16;
    let mut t1 = sx >> 4;
    let mut sy = 0;

    while sx >= sy {
        set_pixel(center + [sx, sy], col);
        sx = -sx;
        set_pixel(center + [sx, sy], col);
        sy = -sy;
        set_pixel(center + [sx, sy], col);
        sx = -sx;
        set_pixel(center + [sx, sy], col);
        (sx, sy) = (sy, sx);
        set_pixel(center + [sx, sy], col);
        sy = -sy;
        set_pixel(center + [sx, sy], col);
        sx = -sx;
        set_pixel(center + [sx, sy], col);
        sy = -sy;
        set_pixel(center + [sx, sy], col);
        (sx, sy) = (sy, sx);

        sy += 1;
        t1 += sy;

        let t2 = t1 - sx;

        if t2 >= 0 {
            t1 = t2;
            sx -= 1;
        }
    }
}

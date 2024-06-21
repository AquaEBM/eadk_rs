#![no_std]

pub mod backlight {

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }

    #[inline]
    pub fn set_brightness(brightness: u8) {
        unsafe {
            eadk_backlight_set_brightness(brightness);
        }
    }

    #[inline]
    pub fn brightness() -> u8 {
        unsafe { eadk_backlight_brightness() }
    }
}

pub mod kandinsky {

    #[repr(C)]
    struct Rect {
        x: i16,
        y: i16,
        w: u16,
        h: u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub struct Color(pub u16);

    impl Color {
        #[inline]
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
        fn eadk_display_wait_for_vblank();
    }

    #[inline]
    pub fn draw_rect(x: i16, y: i16, w: u16, h: u16, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(Rect { x, y, w, h }, pixels.as_ptr());
        }
    }

    #[inline]
    pub fn fill_rect(x: i16, y: i16, w: u16, h: u16, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(Rect { x, y, w, h }, color);
        }
    }

    #[inline]
    pub fn set_pixel(x: i16, y: i16, col: Color) {
        fill_rect(x, y, 1, 1, col);
    }

    #[inline]
    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    pub fn draw_line(mut x0: i16, mut y0: i16, x1: i16, y1: i16, col: Color) {
        let mut dx = x1 - x0;
        let sx = dx.signum();
        dx = dx.abs();

        let mut dy = y1 - y0;
        let sy = dy.signum();
        dy = -dy.abs();

        let mut e = dx + dy;

        loop {
            set_pixel(x0, y0, col);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = e << 1;

            if e2 >= dy {
                if x0 == x1 {
                    break;
                }

                e += dy;
                x0 += sx;
            }

            if e2 <= dx {
                if y0 == y1 {
                    break;
                }

                e += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_circle(x: i16, y: i16, r: u16, col: Color) {
        let mut t1 = r as i16 / 16;
        let mut sx = r as i16;
        let mut sy = 0;

        let draw = |x, y| set_pixel(x, y, col);

        while sx >= sy {
            draw(x + sx, y + sy);
            draw(x - sx, y + sy);
            draw(x + sx, y - sy);
            draw(x - sx, y - sy);
            draw(x + sy, y + sx);
            draw(x - sy, y + sx);
            draw(x + sy, y - sx);
            draw(x - sy, y - sx);

            sy += 1;
            t1 += sy;

            let t2 = t1 - sx;

            if t2 >= 0 {
                t1 = t2;
                sx -= 1;
            }
        }
    }
}

pub mod time {

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }

    #[inline]
    pub fn sleep_us(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    #[inline]
    pub fn sleep_ms(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    #[inline]
    pub fn time_ms() -> u64 {
        unsafe { eadk_timing_millis() }
    }
}

pub mod random {

    extern "C" {
        fn eadk_random() -> u32;
    }

    #[inline]
    pub fn random() -> u32 {
        unsafe { eadk_random() }
    }

    /// Uniformly generates a floating point value in [0 ; 1[
    /// with a precision of 2^(-23)
    #[inline]
    pub fn random_float() -> f32 {
        f32::from_bits(random() >> 9 | 0x3f800000) - 1.
    }
}

pub mod ion {

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
    #[repr(u8)]
    pub enum Key {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Home = 6,
        OnOff = 8,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LParens = 33,
        RParens = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Mult = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
    }

    extern "C" {
        fn eadk_keyboard_scan() -> u64;
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub struct KeyboardState(u64);

    impl KeyboardState {
        pub fn scan() -> Self {
            Self(unsafe { eadk_keyboard_scan() })
        }

        pub fn inner(&self) -> &u64 {
            &self.0
        }

        pub fn key_down(&self, key: Key) -> bool {
            (self.inner() >> (key as u8)) & 1 != 0
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    #[repr(u16)]
    pub enum Event {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LParens = 33,
        RParens = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Mult = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
        ShiftLeft = 54,
        ShiftUp = 55,
        ShiftDown = 56,
        ShiftRight = 57,
        AlphaLock = 67,
        Cut = 68,
        Copy = 69,
        Paste = 70,
        Clear = 71,
        LeftBracket = 72,
        RightBracket = 73,
        LeftBrace = 74,
        RightBrace = 75,
        Underscore = 76,
        Sto = 77,
        Arcsine = 78,
        Arccosine = 79,
        Arctangent = 80,
        Equal = 81,
        Lower = 82,
        Greater = 83,
        Colon = 122,
        Semicolon = 123,
        DoubleQuotes = 124,
        Percent = 125,
        LowerA = 126,
        LowerB = 127,
        LowerC = 128,
        LowerD = 129,
        LowerE = 130,
        LowerF = 131,
        LowerG = 132,
        LowerH = 133,
        LowerI = 134,
        LowerJ = 135,
        LowerK = 136,
        LowerL = 137,
        LowerM = 138,
        LowerN = 139,
        LowerO = 140,
        LowerP = 141,
        LowerQ = 142,
        LowerR = 144,
        LowerS = 145,
        LowerT = 146,
        LowerU = 147,
        LowerV = 148,
        LowerW = 150,
        LowerX = 151,
        LowerY = 152,
        LowerZ = 153,
        Space = 154,
        Question = 156,
        Exclamation = 157,
        UpperA = 180,
        UpperB = 181,
        UpperC = 182,
        UpperD = 183,
        UpperE = 184,
        UpperF = 185,
        UpperG = 186,
        UpperH = 187,
        UpperI = 188,
        UpperJ = 189,
        UpperK = 190,
        UpperL = 191,
        UpperM = 192,
        UpperN = 193,
        UpperO = 194,
        UpperP = 195,
        UpperQ = 196,
        UpperR = 198,
        UpperS = 199,
        UpperT = 200,
        UpperU = 201,
        UpperV = 202,
        UpperW = 204,
        UpperX = 205,
        UpperY = 206,
        UpperZ = 207,
    }

    impl Event {
        pub fn is_digit(&self) -> bool {
            use Event::*;
            [Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine].contains(self)
        }

        pub fn to_digit(&self) -> Option<u8> {
            use Event::*;

            [Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine]
                .iter()
                .find_map(|d| (d == self).then_some(*d as u8))
        }
    }

    extern "C" {
        fn eadk_event_get(timeout: &i32) -> Event;
    }

    pub fn get_event(timeout: i32) -> Event {
        unsafe { eadk_event_get(&timeout) }
    }
}
#![no_std]

pub mod ion;
pub mod kandinsky;

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
    #[must_use]
    pub fn time_ms() -> u64 {
        unsafe { eadk_timing_millis() }
    }
}

pub mod random {

    extern "C" {
        fn eadk_random() -> u32;
    }

    #[inline]
    #[must_use]
    pub fn random() -> u32 {
        unsafe { eadk_random() }
    }

    /// Uniformly generates a floating point value in [0 ; 1[
    /// with a precision of 2^(-23)
    #[inline]
    #[must_use]
    pub fn random_float() -> f32 {
        f32::from_bits(random() >> 9 | 0x3f800000) - 1.
    }
}

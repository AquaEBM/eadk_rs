extern "C" {
    fn eadk_battery_is_charging() -> bool;
    fn eadk_battery_level() -> u8;
    fn eadk_battery_voltage() -> f32;
}

#[inline]
#[must_use]
pub fn is_charging() -> bool {
    unsafe { eadk_battery_is_charging() }
}

#[inline]
#[must_use]
pub fn level() -> u8 {
    unsafe { eadk_battery_level() }
}

#[inline]
#[must_use]
pub fn voltage() -> f32 {
    unsafe { eadk_battery_voltage() }
}

// Adapted from Mish (https://github.com/shingtaklam1324/mish)

use core::f32::{self as f, consts as fc};

const ZERO: f32 = 0.0;
const INF: f32 = f::INFINITY;
const NEG_INF: f32 = f::NEG_INFINITY;
const HALF: f32 = 0.5;
const ONE: f32 = 1.0;
const TWO: f32 = 2.0;
const LN2: f32 = fc::LN_2;
const LN10: f32 = fc::LN_10;
const PI: f32 = fc::PI;
const PI2: f32 = fc::FRAC_PI_2;

const C6: f32 = 6.0;
const C120: f32 = 120.0;
const C5040: f32 = 5040.0;
const C362880: f32 = 362880.0;
const C3: f32 = 3.0;
const C2D15: f32 = 2.0 / 15.0;
const C17D315: f32 = 17.0 / 315.0;

use core::intrinsics;

#[inline]
pub fn powi(x: f32, i: i32) -> f32 {
    unsafe { intrinsics::powif32(x, i) }
}

#[inline]
pub fn fabs(x: f32) -> f32 {
    unsafe { intrinsics::fabsf32(x) }
}

#[inline]
pub fn sqrt(x: f32) -> f32 {
    unsafe { intrinsics::sqrtf32(x) }
}

#[inline]
pub fn sin(x: f32) -> f32 {
    x - powi(x, 3) / C6 + powi(x, 5) / C120 - powi(x, 7) / C5040 + powi(x, 9) / C362880
}

#[inline]
pub fn cos(x: f32) -> f32 {
    sin(PI2 - x)
}

#[inline]
pub fn clamp(min: f32, value: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[inline]
pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

pub fn floor(f: f32) -> f32 {
    unsafe { intrinsics::floorf32(f) }
}

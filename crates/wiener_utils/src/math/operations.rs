use std::ops::AddAssign;

use num::traits::{real::Real, Pow};
use num::{Float, Num};

/// Calculate the cross product between two vectors.
pub fn cross<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: std::ops::Mul<T, Output = T> + std::ops::Sub<T, Output = T> + Copy,
{
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
}

/// Normalize a vector
pub fn normalize<T>(a: &[T]) -> Vec<T>
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let length = {
        let mut result: T = T::zero();
        for i in a {
            result += i.pow(2);
        }
        result.sqrt()
    };
    let mut result = Vec::new();
    for i in a {
        result.push(*i / length);
    }
    return result;
}

/// Transform from spherical to cartesian coordinates.
pub fn spherical_to_cartesian<T: Float>(r: T, phi: T, theta: T) -> [T; 3] {
    return [
        r * theta.sin() * phi.cos(),
        r * theta.cos(),
        r * theta.sin() * phi.sin(),
    ];
}

/// Clamp a number between two limits.
pub fn clamp<T: PartialOrd<T> + Copy>(num: T, bottom: Option<T>, top: Option<T>) -> T {
    if top.is_some() && num > top.unwrap() {
        return top.unwrap();
    }
    else if bottom.is_some() && num < bottom.unwrap() {
        return bottom.unwrap();
    }
    else {
        return num;
    }
}

/// Generate perspective matrix.
pub fn perspective_mat<T: Float>(n: T, f: T, l: T, r: T, t: T, b: T) -> [[T; 4]; 4] {
    let two = T::from(2.0).expect("Failed to convert to float");
    return [
        [two * n / (r - l), T::zero(), (r + l) / (r - l), T::zero()],
        [T::zero(), two * n / (t - b), (t + b) / (t - b), T::zero()],
        [T::zero(), T::zero(), (n + f) / (n - f), two * n * f / (n - f)],
        [T::zero(), T::zero(), -T::one(), T::zero()],
    ]
}

/// Generate ortographic matrix.
pub fn ortographic_mat<T: Float>(n: T, f: T, l: T, r: T, t: T, b: T) -> [[T; 4]; 4] {
    let two = T::from(2.0).expect("Failed to convert to float");
    return [
        [two / (r - l), T::zero(), T::zero(), (l + r) / (l - r)],
        [T::zero(), two / (t - b), T::zero(), (b + t) / (b - t)],
        [T::zero(), T::zero(), two / (n - f), (n + f) / (n - f)],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ]
}

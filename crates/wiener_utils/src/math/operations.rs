use std::ops::{AddAssign, Sub};

use num::traits::{real::Real, Pow};
use num::{Float, Num};

/// Subtract two vectors.
pub fn subtract2<T: Sub<T, Output = T> + Copy>(a: [T; 2], b: [T; 2]) -> [T; 2] {
    return [a[0] - b[0], a[1] - b[1]];
}

/// Subtract two vectors.
pub fn subtract3<T: Sub<T, Output = T> + Copy>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

/// Subtract two vectors.
pub fn subtract4<T: Sub<T, Output = T> + Copy>(a: [T; 4], b: [T; 4]) -> [T; 4] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]];
}

/// Extract a column from a matrix.
pub fn extract_column2<T: Num + Copy>(mat: [[T; 2]; 2], index: usize) -> [T; 2] {
    let mut result = [T::zero(), T::zero()];
    for i in 0..mat.len() {
        result[i] = mat[i][index];
    }
    return result;
}

/// Extract a column from a matrix.
pub fn extract_column3<T: Num + Copy>(mat: [[T; 3]; 3], index: usize) -> [T; 3] {
    let mut result = [T::zero(), T::zero(), T::zero()];
    for i in 0..mat.len() {
        result[i] = mat[i][index];
    }
    return result;
}

/// Extract a column from a matrix.
pub fn extract_column4<T: Num + Copy>(mat: [[T; 4]; 4], index: usize) -> [T; 4] {
    let mut result = [T::zero(), T::zero(), T::zero(), T::zero()];
    for i in 0..mat.len() {
        result[i] = mat[i][index];
    }
    return result;
}

/// Multiply two matrices.
pub fn matmul2<T: Num + Copy + AddAssign<T>>(a: [[T; 2]; 2], b: [[T; 2]; 2]) -> [[T; 2]; 2] {
    let mut result = [[T::zero(); 2]; 2];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i][j] = dot2(a[i], extract_column2(b, j));
        }
    }
    return result;
}

/// Multiply two matrices.
pub fn matmul3<T: Num + Copy + AddAssign<T>>(a: [[T; 3]; 3], b: [[T; 3]; 3]) -> [[T; 3]; 3] {
    let mut result = [[T::zero(); 3]; 3];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i][j] = dot3(a[i], extract_column3(b, j));
        }
    }
    return result;
}

/// Multiply two matrices.
pub fn matmul4<T: Num + Copy + AddAssign<T>>(a: [[T; 4]; 4], b: [[T; 4]; 4]) -> [[T; 4]; 4] {
    let mut result = [[T::zero(); 4]; 4];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i][j] = dot4(a[i], extract_column4(b, j));
        }
    }
    return result;
}

/// Multiply a matrix and a vector.
pub fn matmul2v<T: Num + Copy + AddAssign<T>>(mat: [[T; 2]; 2], v: [T; 2]) -> [T; 2] {
    let mut result = [T::zero(); 2];
    for i in 0..mat.len() {
        result[i] = dot2(mat[i], v);
    }
    return result;
}

/// Multiply a matrix and a vector.
pub fn matmul3v<T: Num + Copy + AddAssign<T>>(mat: [[T; 3]; 3], v: [T; 3]) -> [T; 3] {
    let mut result = [T::zero(); 3];
    for i in 0..mat.len() {
        result[i] = dot3(mat[i], v);
    }
    return result;
}

/// Multiply a matrix and a vector.
pub fn matmul4v<T: Num + Copy + AddAssign<T>>(mat: [[T; 4]; 4], v: [T; 4]) -> [T; 4] {
    let mut result = [T::zero(); 4];
    for i in 0..mat.len() {
        result[i] = dot4(mat[i], v);
    }
    return result;
}

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

/// Calculate the norm of a vector.
pub fn norm2<T>(v: [T; 2]) -> T
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let mut result: T = T::zero();
    for i in v {
        result += i.pow(2);
    }
    return result.sqrt();
}

/// Calculate the norm of a vector.
pub fn norm3<T>(v: [T; 3]) -> T
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let mut result: T = T::zero();
    for i in v {
        result += i.pow(2);
    }
    return result.sqrt();
}

/// Calculate the norm of a vector.
pub fn norm4<T>(v: [T; 4]) -> T
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let mut result: T = T::zero();
    for i in v {
        result += i.pow(2);
    }
    return result.sqrt();
}

/// Normalize a vector
pub fn normalize2<T>(a: [T; 2]) -> [T; 2]
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let mut length = norm2(a);
    if length == T::zero() {
        length = T::one();
    }
    let mut result = [T::zero(), T::zero()];
    for i in 0..a.len() {
        result[i] = a[i] / length;
    }
    return result;
}

/// Normalize a vector
pub fn normalize3<T>(a: [T; 3]) -> [T; 3]
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let mut length = norm3(a);
    if length == T::zero() {
        length = T::one();
    }
    let mut result = [T::zero(), T::zero(), T::zero()];
    for i in 0..a.len() {
        result[i] = a[i] / length;
    }
    return result;
}

/// Normalize a vector
pub fn normalize4<T>(a: [T; 4]) -> [T; 4]
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let mut length = norm4(a);
    if length == T::zero() {
        length = T::one();
    }
    let mut result = [T::zero(), T::zero(), T::zero(), T::zero()];
    for i in 0..a.len() {
        result[i] = a[i] / length;
    }
    return result;
}

/// Calculate the dot product between two vectors.
pub fn dot2<T>(a: [T; 2], b: [T; 2]) -> T
where
    T: Num + AddAssign<T> + Copy,
{
    let mut result = T::zero();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    return result;
}

/// Calculate the dot product between two vectors.
pub fn dot3<T>(a: [T; 3], b: [T; 3]) -> T
where
    T: Num + AddAssign<T> + Copy,
{
    let mut result = T::zero();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    return result;
}

/// Calculate the dot product between two vectors.
pub fn dot4<T>(a: [T; 4], b: [T; 4]) -> T
where
    T: Num + AddAssign<T> + Copy,
{
    let mut result = T::zero();
    for i in 0..a.len() {
        result += a[i] * b[i];
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
    } else if bottom.is_some() && num < bottom.unwrap() {
        return bottom.unwrap();
    } else {
        return num;
    }
}

/// Generate perspective matrix.
pub fn perspective_mat<T: Real>(n: T, f: T, l: T, r: T, t: T, b: T) -> [[T; 4]; 4] {
    let two = T::from(2.0).expect("Failed to convert to float");
    return [
        [two * n / (r - l), T::zero(), (r + l) / (r - l), T::zero()],
        [T::zero(), two * n / (t - b), (t + b) / (t - b), T::zero()],
        [
            T::zero(),
            T::zero(),
            (n + f) / (n - f),
            two * n * f / (n - f),
        ],
        [T::zero(), T::zero(), -T::one(), T::zero()],
    ];
}

/// Generate perspective matrix from an FOV.
pub fn perspective_fov_mat<T: Real>(fov: T, aspect: T, near: T, far: T) -> [[T; 4]; 4] {
    let top = near * fov.sin();
    let right = top / aspect;
    let left = -right;
    let bottom = -top;
    return perspective_mat(near, far, left, right, top, bottom);
}

/// Generate ortographic matrix.
pub fn ortographic_mat<T: Real>(n: T, f: T, l: T, r: T, t: T, b: T) -> [[T; 4]; 4] {
    let two = T::from(2.0).expect("Failed to convert to float");
    return [
        [two / (r - l), T::zero(), T::zero(), (l + r) / (l - r)],
        [T::zero(), two / (t - b), T::zero(), (b + t) / (b - t)],
        [T::zero(), T::zero(), two / (n - f), (n + f) / (n - f)],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ];
}

/// Generate a view matrix.
pub fn view_mat<T>(eye: [T; 3], up: [T; 3], at: [T; 3]) -> [[T; 4]; 4]
where
    T: Num + Pow<u16, Output = T> + AddAssign<T> + Real,
{
    let e = eye;
    let a = at;
    let f = normalize3(subtract3(a, e));
    let s = cross(f, normalize3(up));
    let u = cross(s, f);
    return [
        [s[0], s[1], s[2], -dot3(e, s)],
        [u[0], u[1], u[2], -dot3(e, u)],
        [-f[0], -f[1], -f[2], dot3(e, f)],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ];
}

/// Generate a translation matrix.
pub fn translation<T: Num>(x: T, y: T, z: T) -> [[T; 4]; 4] {
    return [
        [T::one(), T::zero(), T::zero(), x],
        [T::zero(), T::one(), T::zero(), y],
        [T::zero(), T::zero(), T::one(), z],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ];
}

/// Generate a scaling matrix.
pub fn scaling<T: Num>(x: T, y: T, z: T) -> [[T; 4]; 4] {
    return [
        [x, T::zero(), T::zero(), T::zero()],
        [T::zero(), y, T::zero(), T::zero()],
        [T::zero(), T::zero(), z, T::zero()],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ];
}

/// Generate a rotation matrix.
pub fn rotation<T: Num + Real>(x: T, y: T, z: T) -> [[T; 4]; 4] {
    return [
        [
            y.cos() * z.cos(),
            x.sin() * y.sin() * z.cos() - x.cos() * z.sin(),
            x.cos() * y.sin() * z.cos() + x.sin() * z.sin(),
            T::zero(),
        ],
        [
            y.cos() * z.sin(),
            x.sin() * y.sin() * z.sin() + x.cos() * z.cos(),
            x.cos() * y.sin() * z.sin() - x.sin() * z.cos(),
            T::zero(),
        ],
        [-y.sin(), x.sin() * y.cos(), x.cos() * y.cos(), T::zero()],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ];
}

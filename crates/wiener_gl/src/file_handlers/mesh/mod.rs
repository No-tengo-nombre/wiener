use num::{traits::Pow, Float, ToPrimitive};
use std::fmt::Debug;
use std::ops::AddAssign;
use std::str::FromStr;

mod file_obj;
mod file_off;

pub use file_obj::*;
pub use file_off::*;

pub trait MeshFileHandler {
    fn load_file<
        U: Float + From<f32> + FromStr + AddAssign<U> + Pow<u16, Output = U>,
        I: FromStr + ToPrimitive + Copy,
    >(
        &self,
    ) -> (Vec<U>, Vec<I>, u32)
    where
        <U as FromStr>::Err: Debug,
        <I as FromStr>::Err: Debug;

    fn get_name<'a>() -> &'a str;
}

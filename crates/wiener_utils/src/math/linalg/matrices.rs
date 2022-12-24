use num::Num;



pub fn eye1<T: Num>() -> [[T; 1]; 1] {
    return [
        [T::one()],
    ];
}

pub fn eye2<T: Num>() -> [[T; 2]; 2] {
    return [
        [T::one(), T::zero()],
        [T::zero(), T::one()],
    ];
}

pub fn eye3<T: Num>() -> [[T; 3]; 3] {
    return [
        [T::one(), T::zero(), T::zero()],
        [T::zero(), T::one(), T::zero()],
        [T::zero(), T::zero(), T::one()],
    ];
}

pub fn eye4<T: Num>() -> [[T; 4]; 4] {
    return [
        [T::one(), T::zero(), T::zero(), T::zero()],
        [T::zero(), T::one(), T::zero(), T::zero()],
        [T::zero(), T::zero(), T::one(), T::zero()],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ];
}

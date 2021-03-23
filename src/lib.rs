// TODO Ideally we should have pub(create) visibility, so that these functions are visible only
// from inside this crate. However all the binaries are in src/bin/ which (probably) constitutes
// another crate, so it can't see math_utils this way.
//
// pub(crate) mod math_utils;

pub mod math_utils;
pub mod data_structs;

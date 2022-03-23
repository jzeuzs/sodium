#[macro_export]
macro_rules! vec_arr_func {
    ($name:ident, $type:ty, $size:expr) => {
        pub fn $name(data: &[$type]) -> [$type; $size] {
            let mut arr = [0; $size];
            arr.copy_from_slice(&data[0..$size]);
            arr
        }
    };
}

#[macro_export]
macro_rules! create_arr_with_length {
    ($size:expr) => {{
        let a: [u8; $size] = [0; $size];

        a
    }};
}

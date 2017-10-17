
#[macro_export]
macro_rules! c_struct {
    ($name:ident, $wrap:ty, $create:ident => $( $params:ident ),* => $new:expr => $free:ident) => {
        #[repr(C)]
        pub struct $name(pub $wrap);

        #[no_mangle]
        pub fn $create($($params ,)*) -> *mut $name {
            let val = $name($new);
            let val = Box::new(val);

            Box::into_raw(val)
        }

        #[no_mangle]
        pub fn $free(val: *mut $name) {
            unsafe { Box::from_raw(val); }
        }
    };
}

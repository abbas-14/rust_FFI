use libc::{c_int};

#[no_mangle]
extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn sub(a: c_int, b: c_int) -> c_int;
    fn mul(a: c_int, b: c_int) -> c_int;
    fn div(a: c_int, b: c_int) -> c_int;
    fn modulo(a: c_int, b: c_int) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Numerics::test;

    #[test]
    fn it_works() {
        let x: c_int = 100;
        let y: c_int = 25;
        
        let r = unsafe { test::<c_int>("add", x, y, 0) };
        assert_eq!(r, 125);
        
        let r = unsafe { test::<c_int>("sub", x, y, 0) };
        assert_eq!(r, 75);
        
        let r = unsafe { test::<c_int>("mul", x, y, 0) };
        assert_eq!(r, 2500);
        
        let r = unsafe { test::<c_int>("div", x, y, 0) };
        assert_eq!(r, 4);
        
        let r = unsafe { test::<c_int>("modulo", x, y, 0) };
        assert_eq!(r, 0);
    }
}
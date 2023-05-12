use libc::c_float;

#[no_mangle]
extern "C" {
    fn add(a: c_float, b: c_float) -> c_float;
    fn sub(a: c_float, b: c_float) -> c_float;
    fn mul(a: c_float, b: c_float) -> c_float;
    fn div(a: c_float, b: c_float) -> c_float;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Numerics::test;

    #[test]
    fn it_works() {
        let x: c_float = 100.0;
        let y: c_float = 30.0;
        
        let r = unsafe { test::<c_float>("add", x, y, 1) };
        assert_eq!(r, x + y);
        
        let r = unsafe { test::<c_float>("sub", x, y, 1) };
        assert_eq!(r, x - y);
        
        let r = unsafe { test::<c_float>("mul", x, y, 1) };
        assert_eq!(r, x * y);
        
        let r = unsafe { test::<c_float>("div", x, y, 1) };
        assert_eq!(r, x / y);
    }
}
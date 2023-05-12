use libc::{c_char, c_short};


#[no_mangle]
extern "C" {
    fn str_rev(s: *const c_char) -> *mut c_char;
    fn str_cmp(s1: *const c_char, s2: *const c_char) -> c_short;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::paths;
    use std::ffi::{CStr, CString};
    use libloading::{Library, Symbol};

    #[test]
    fn string_reversal_works() {
        unsafe {
            let lib = Library::new(paths[2]).unwrap();
            let func: Symbol<
                unsafe extern "C" fn(*const c_char) -> *mut c_char
            > = lib.get(b"str_rev").unwrap();
            
            let s: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            let xpected: *mut c_char = CString::new("desrever gnieb ma i").unwrap().into_raw();

            assert_eq!(
                CStr::from_ptr(func(s)), 
                CStr::from_ptr(xpected), 
                "xpected result: {:?}", CStr::from_ptr(xpected)
            ); 
        };
    }

    #[test]
    fn string_comparision_works() {
         unsafe {
            let lib = Library::new(paths[2]).unwrap();
            let func: Symbol<
                unsafe extern "C" fn(*const c_char, *const c_char) -> c_short
            > = lib.get(b"str_cmp").unwrap();
            
            // first = second
            let s1: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            let s2: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            assert_eq!(
                func(s1, s2), 
                0, 
                "xpected result: {:?}", 0
            );
            
            // first > second, by len
            let s1: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            let s2: *const c_char = CString::new("i am being reverse").unwrap().into_raw();
            assert_eq!(
                func(s1, s2), 
                1, 
                "xpected result: {:?}", 1
            );
            
            // first < second, by len
            let s1: *const c_char = CString::new("i am being reverse").unwrap().into_raw();
            let s2: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            assert_eq!(
                func(s1, s2), 
                -1, 
                "xpected result: {:?}", -1
            );
            
            // first > second, by ascii
            let s1: *const c_char = CString::new("i bm being reversed").unwrap().into_raw();
            let s2: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            assert_eq!(
                func(s1, s2), 
                1, 
                "xpected result: {:?}", 1
            );
            // first < second, by ascii
            let s1: *const c_char = CString::new("i am being reversed").unwrap().into_raw();
            let s2: *const c_char = CString::new("i an being reversed").unwrap().into_raw();
            assert_eq!(
                func(s1, s2),
                -1, 
                "xpected result: {:?}", -1
            );
        };
    }
}
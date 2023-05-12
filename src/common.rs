use libloading::{Library, Symbol};

pub const paths: [&str; 3] = [
    "./src/ints/ints.dylib",
    "./src/floats/floats.dylib",
    "./src/strings/strings.dylib",
];

pub mod Numerics {
    use super::*;
    
    pub fn test<T>(func: &str, x: T, y: T, i: usize) -> T {
        unsafe {
            let lib = Library::new(paths[i]).unwrap();
            let func: Symbol<
                unsafe extern "C" fn(T, T) -> T
            > = lib.get(func.as_bytes()).unwrap();
            func(x, y)
        }
    }
}
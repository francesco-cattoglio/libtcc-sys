#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use tcc_new;
        assert_eq!(2 + 2, 4);
        unsafe {
            let _s = tcc_new();
        }
    }
}

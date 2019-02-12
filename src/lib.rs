#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        use tcc_new;
        use tcc_set_output_type;
        use tcc_compile_string;
        use tcc_relocate;
        use tcc_get_symbol;
        use TCC_OUTPUT_MEMORY;
        use std::ffi;
        use std::ptr;

        let c_source = ffi::CString::new(r#"
#include <stdio.h> 
#ifdef _WIN32 /* dynamically linked data needs 'dllimport' */
 __attribute__((dllimport))
#endif
void func_hello()
{
    printf("hello world from JITted C!\n");
}
        "#).unwrap();
        assert_eq!(2 + 2, 4);
        unsafe {
            let s = tcc_new();
            assert!(!s.is_null());
            tcc_set_output_type(s, TCC_OUTPUT_MEMORY as i32);

            /* compile and relocate the code */
            let compile_result = tcc_compile_string(s, c_source.as_ptr());
            assert!(compile_result != -1);
            // libtcc defines TCC_RELOCATE_AUTO as (void*) 1
            let terrible_hack = ptr::null_mut::<ffi::c_void>();
            let relocate_result = tcc_relocate(s, terrible_hack.offset(1));
            assert!(relocate_result >= 0);

            let symbol_name = ffi::CString::new("func_hello").unwrap();
            let loaded_func = tcc_get_symbol(s, symbol_name.as_ptr());
            let transmuted = std::mem::transmute::<*const ffi::c_void, fn() -> ()>(loaded_func);

            transmuted();
        }
    }
}

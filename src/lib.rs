#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        use tcc_new;
        use tcc_delete;
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
    printf("Hello world from JIT compiled C! ... ");
    fflush(stdout);
}
        "#).unwrap();
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
            let transmuted = std::mem::transmute::<*const ffi::c_void, extern "C" fn() -> ()>(loaded_func);

            transmuted();

            tcc_delete(s);
        }
    }

    #[test]
    fn float_test() {
        use tcc_new;
        use tcc_delete;
        use tcc_set_output_type;
        use tcc_compile_string;
        use tcc_relocate;
        use tcc_get_symbol;
        use TCC_OUTPUT_MEMORY;
        use std::ffi;
        use std::ptr;

        let c_source = ffi::CString::new(r#"
#include <math.h> 
#ifdef _WIN32 /* dynamically linked data needs 'dllimport' */
 __attribute__((dllimport))
#endif
// cos^2 + sin^2 should always return one,
// (unless x is a very large float, then some rounding will happen!)
float func_one(float x)
{
    return cosf(x)*cosf(x) + sinf(x)*sinf(x);
}
        "#).unwrap();
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

            let symbol_name = ffi::CString::new("func_one").unwrap();
            let loaded_func = tcc_get_symbol(s, symbol_name.as_ptr());
            let transmuted = std::mem::transmute::<*const ffi::c_void, extern "C" fn(f32) -> f32>(loaded_func);

            assert_eq!(transmuted(42.0f32), 1.0f32);
            tcc_delete(s);
        }
    }

    #[test]
    fn owned_memory_test() {
        use tcc_new;
        use tcc_delete;
        use tcc_set_output_type;
        use tcc_compile_string;
        use tcc_relocate;
        use tcc_get_symbol;
        use TCC_OUTPUT_MEMORY;
        use std::ffi;
        use std::ptr;

        let c_source = ffi::CString::new(r#"
#include <math.h> 
#ifdef _WIN32 /* dynamically linked data needs 'dllimport' */
 __attribute__((dllimport))
#endif
int func_add(int a, int b)
{
    return a + b;
}
        "#).unwrap();
        unsafe {
            let s = tcc_new();
            assert!(!s.is_null());
            tcc_set_output_type(s, TCC_OUTPUT_MEMORY as i32);

            /* compile and relocate the code */
            let compile_result = tcc_compile_string(s, c_source.as_ptr());
            assert!(compile_result != -1);
            // libtcc defines TCC_RELOCATE_AUTO as (void*) 1
            let null_ptr = ptr::null_mut::<ffi::c_void>();
            let required_memory = tcc_relocate(s, null_ptr);

            //  memory_buffer
            let mut memory_buffer : Vec<u8> = Vec::new();
            memory_buffer.resize(required_memory as usize, 0u8);

            let pass_to_c = std::mem::transmute::<*mut u8, *mut std::ffi::c_void>(memory_buffer.as_mut_ptr());
            let relocate_result = tcc_relocate(s, pass_to_c);
            assert!(relocate_result >= 0);

            use std::os::raw::c_int;
            let symbol_name = ffi::CString::new("func_add").unwrap();
            let loaded_func = tcc_get_symbol(s, symbol_name.as_ptr());
            // we are managing our own memory, therefore even if we delete the tcc status, the
            // function will still exist in our memory_buffer.
            tcc_delete(s);

            let transmuted = std::mem::transmute::<*const ffi::c_void, extern "C" fn(c_int, c_int) -> c_int>(loaded_func);
            assert_eq!(transmuted(17, 25), 42);
        }
    }
}

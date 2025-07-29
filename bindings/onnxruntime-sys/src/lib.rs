#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe impl Send for OrtApi {}
unsafe impl Sync for OrtApi {}

unsafe impl Send for OrtValue {}

unsafe impl Send for OrtEnv {}

unsafe impl Send for OrtSession {}

unsafe impl Send for OrtMemoryInfo {}

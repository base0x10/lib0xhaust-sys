#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

// The bindgen generated tests have a known issue in the allignment tests
// where they play around with null pointers.  This UB is only generated
// during cargo test
#![allow(deref_nullptr)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

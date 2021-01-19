#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! # Info
//! Raw bindings to Houdini Engine C API
//! A much more nicer, Rusty crate hapi-rs is using these bindings and currently in WIP.
//!
//! There's a feature flag available called `rustify` which makes enums and their variants less ugly,
//! for example `HAPI_Result::HAPI_RESULT_SUCCESS` becomes `HapiResult::Success`
//!
//! # Building
//! HFS variable must be set, which is used to find Houdini header files and libraries
//!
//! # Running Tests
//! `env LD_LIBRARY_PATH=$HDSO cargo test`  // on Linux
//!
//! `env DYLD_FALLBACK_LIBRARY_PATH=$HDSO cargo test` // on Mac
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
/// A simple test to initialize a session.
/// Require libHAPI to run:
/// Linux:
/// MacOS:
mod test {
    use std::mem::MaybeUninit;

    use super::*;

    #[test]
    fn basic_test() {
        unsafe {
            let mut ses = MaybeUninit::uninit();
            let res = HAPI_CreateInProcessSession(ses.as_mut_ptr());
            assert_eq!(res, HAPI_Result::HAPI_RESULT_SUCCESS);
            let ses = ses.assume_init();
            assert_eq!(ses.type_, HAPI_SessionType::HAPI_SESSION_INPROCESS);
        }
    }
}

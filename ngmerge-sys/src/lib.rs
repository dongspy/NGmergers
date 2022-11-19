#![allow(non_snake_case, non_camel_case_types)]
use std::env;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

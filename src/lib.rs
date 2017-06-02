#![allow(non_camel_case_types, non_snake_case)]
extern crate libc;

pub mod lua;
pub mod luaconf;
pub mod lauxlib;
pub mod lualib;

mod glue {
    include!(concat!(env!("OUT_DIR"), "/glue.rs"));
}

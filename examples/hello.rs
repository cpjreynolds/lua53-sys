extern crate lua53_sys;
extern crate libc;

use lua53_sys::{lauxlib, lua};

pub const HELLO: &'static [u8] = b"print(\"Hello\")\0";

fn main() {
    unsafe {
        let state = lauxlib::luaL_newstate();
        lauxlib::luaL_openlibs(state);
        lauxlib::luaL_dostring(state, HELLO.as_ptr() as *const libc::c_char);
        lua::lua_close(state);
    }
}

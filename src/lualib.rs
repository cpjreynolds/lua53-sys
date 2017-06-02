//! Definitions from `lualib.h`.

use libc::c_int;
use lua::lua_State;

pub use super::glue::{LUA_COLIBNAME, LUA_TABLIBNAME, LUA_IOLIBNAME, LUA_OSLIBNAME, LUA_STRLIBNAME,
                      LUA_UTF8LIBNAME, LUA_BITLIBNAME, LUA_MATHLIBNAME, LUA_DBLIBNAME,
                      LUA_LOADLIBNAME};

extern "C" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_utf8(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;
}

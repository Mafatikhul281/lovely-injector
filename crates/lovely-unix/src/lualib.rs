use std::sync::LazyLock;

use libloading::Library;

use lovely_core::sys::LuaLib;

pub static LUA_LIBRARY: LazyLock<Library> = LazyLock::new(|| unsafe {
    #[cfg(target_os = "macos")]
    return Library::new("../Frameworks/Lua.framework/Versions/A/Lua").unwrap();
    #[cfg(target_os = "linux")]
    return Library::new("libluajit-5.1.so.2").unwrap();
    #[cfg(target_os = "android")]
    return Library::new("libluajit.so").unwrap();
});

pub unsafe fn get_lualib() -> LuaLib {
    LuaLib::from_library(&LUA_LIBRARY)
}

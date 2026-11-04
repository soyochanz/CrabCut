//! Standalone functions that have no associated type.

use std::ffi::{c_int, CStr, CString};

/// Return the id of the specified language, returns -1 if not found
///
/// # Arguments
/// * lang: The language to get the id for.
///
/// # Returns
/// The ID of the language, None if not found.
///
/// # Panics
/// Panics if the language contains a null byte.
///
/// # C++ equivalent
/// `int whisper_lang_id(const char * lang)`
pub fn get_lang_id(lang: &str) -> Option<c_int> {
    let c_lang = CString::new(lang).expect("Language contains null byte");
    let ret = unsafe { whisper_rs_sys::whisper_lang_id(c_lang.as_ptr()) };
    if ret == -1 {
        None
    } else {
        Some(ret)
    }
}

/// Return the ID of the maximum language (ie the number of languages - 1)
///
/// # Returns
/// i32
///
/// # C++ equivalent
/// `int whisper_lang_max_id()`
pub fn get_lang_max_id() -> i32 {
    unsafe { whisper_rs_sys::whisper_lang_max_id() }
}

/// Get the short string of the specified language id (e.g. 2 -> "de").
///
/// # Returns
/// The short string of the language, None if not found.
///
/// # C++ equivalent
/// `const char * whisper_lang_str(int id)`
pub fn get_lang_str(id: i32) -> Option<&'static str> {
    let c_buf = unsafe { whisper_rs_sys::whisper_lang_str(id) };
    if c_buf.is_null() {
        None
    } else {
        let c_str = unsafe { CStr::from_ptr(c_buf) };
        Some(c_str.to_str().unwrap())
    }
}

/// Get the full string of the specified language name (e.g. 2 -> "german").
///
/// # Returns
/// The full string of the language, None if not found.
///
/// # C++ equivalent
/// `const char * whisper_lang_str_full(int id)`
pub fn get_lang_str_full(id: i32) -> Option<&'static str> {
    let c_buf = unsafe { whisper_rs_sys::whisper_lang_str_full(id) };
    if c_buf.is_null() {
        None
    } else {
        let c_str = unsafe { CStr::from_ptr(c_buf) };
        Some(c_str.to_str().unwrap())
    }
}

/// Callback to control logging output: default behaviour is to print to stderr.
///
/// # Safety
/// The callback must be safe to call from C (i.e. no panicking, no unwinding, etc).
///
/// # C++ equivalent
/// `void whisper_set_log_callback(whisper_log_callback callback);`
pub unsafe fn set_log_callback(
    log_callback: crate::WhisperLogCallback,
    user_data: *mut std::ffi::c_void,
) {
    unsafe {
        whisper_rs_sys::whisper_log_set(log_callback, user_data);
    }
}

/// Get the current `whisper.cpp` version.
pub fn get_whisper_version() -> &'static str {
    let ptr = unsafe { whisper_rs_sys::whisper_version() };
    assert!(!ptr.is_null());
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .expect("Whisper version should be valid UTF-8")
}

/// Print system information.
///
/// # C++ equivalent
/// `const char * whisper_print_system_info()`
pub fn print_system_info() -> &'static str {
    let c_buf = unsafe { whisper_rs_sys::whisper_print_system_info() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    c_str.to_str().unwrap()
}

/// Programmatically exposes the information provided by `print_system_info`
///
/// # C++ equivalent
/// `int ggml_cpu_has_...`
#[derive(Debug)]
pub struct SystemInfo {
    pub avx: bool,
    pub avx2: bool,
    pub fma: bool,
    pub f16c: bool,
}

// CrabCut: CPU kernels may be loaded as ggml backend modules. Query the
// registered backend's public feature API rather than linking directly to
// functions that only exist inside one particular CPU module.
impl Default for SystemInfo {
    fn default() -> Self {
        let mut result = Self {
            avx: false,
            avx2: false,
            fma: false,
            f16c: false,
        };
        unsafe {
            let reg = whisper_rs_sys::ggml_backend_reg_by_name(c"CPU".as_ptr());
            if reg.is_null() {
                return result;
            }
            let proc = whisper_rs_sys::ggml_backend_reg_get_proc_address(
                reg,
                c"ggml_backend_get_features".as_ptr(),
            );
            if proc.is_null() {
                return result;
            }
            // ggml-backend.h defines this exact signature for the named proc.
            let get_features: unsafe extern "C" fn(
                whisper_rs_sys::ggml_backend_reg_t,
            )
                -> *mut whisper_rs_sys::ggml_backend_feature = std::mem::transmute(proc);
            let mut feature = get_features(reg);
            if feature.is_null() {
                return result;
            }
            while !(*feature).name.is_null() {
                let name = std::ffi::CStr::from_ptr((*feature).name).to_bytes();
                let enabled = !(*feature).value.is_null()
                    && std::ffi::CStr::from_ptr((*feature).value).to_bytes() == b"1";
                match name {
                    b"AVX" => result.avx = enabled,
                    b"AVX2" => result.avx2 = enabled,
                    b"FMA" => result.fma = enabled,
                    b"F16C" => result.f16c = enabled,
                    _ => {}
                }
                feature = feature.add(1);
            }
        }
        result
    }
}


//! Environment module - Rust equivalent of environ/environ.c
//! Contains the global pojav_environ structure with all runtime state

use jni::sys::{jboolean, jbyte, jint, jlong};
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

/// Event types for input handling
pub const EVENT_TYPE_CHAR: i32 = 1000;
pub const EVENT_TYPE_CHAR_MODS: i32 = 1001;
pub const EVENT_TYPE_CURSOR_ENTER: i32 = 1002;
pub const EVENT_TYPE_CURSOR_POS: i32 = 1003;
pub const EVENT_TYPE_FRAMEBUFFER_SIZE: i32 = 1004;
pub const EVENT_TYPE_KEY: i32 = 1005;
pub const EVENT_TYPE_MOUSE_BUTTON: i32 = 1006;
pub const EVENT_TYPE_SCROLL: i32 = 1007;
pub const EVENT_TYPE_WINDOW_SIZE: i32 = 1008;

/// Maximum number of events in queue
pub const EVENT_WINDOW_SIZE: usize = 8000;

/// Input event structure
#[derive(Clone, Copy)]
pub struct GLFWInputEvent {
    pub type_: i32,
    pub i1: i32,
    pub i2: i32,
    pub i3: i32,
    pub i4: i32,
}

/// Callback function types
pub type GLFW_invoke_Char_func = Option<unsafe fn(window: *mut libc::c_void, codepoint: u32)>;
pub type GLFW_invoke_CharMods_func = Option<unsafe fn(window: *mut libc::c_void, codepoint: u32, mods: i32)>;
pub type GLFW_invoke_CursorEnter_func = Option<unsafe fn(window: *mut libc::c_void, entered: i32)>;
pub type GLFW_invoke_CursorPos_func = Option<unsafe fn(window: *mut libc::c_void, xpos: f64, ypos: f64)>;
pub type GLFW_invoke_FramebufferSize_func = Option<unsafe fn(window: *mut libc::c_void, width: i32, height: i32)>;
pub type GLFW_invoke_Key_func = Option<unsafe fn(window: *mut libc::c_void, key: i32, scancode: i32, action: i32, mods: i32)>;
pub type GLFW_invoke_MouseButton_func = Option<unsafe fn(window: *mut libc::c_void, button: i32, action: i32, mods: i32)>;
pub type GLFW_invoke_Scroll_func = Option<unsafe fn(window: *mut libc::c_void, xoffset: f64, yoffset: f64)>;
pub type GLFW_invoke_WindowSize_func = Option<unsafe fn(window: *mut libc::c_void, width: i32, height: i32)>;

/// Global environment state
static POJAV_ENVIRON: AtomicPtr<PojavEnviron> = AtomicPtr::new(ptr::null_mut());

/// Main environment structure (simplified, raw pointers for JNI compatibility)
#[repr(C)]
pub struct PojavEnviron {
    pub pojav_window: *mut libc::c_void,
    pub main_window_bundle: *mut libc::c_void,
    pub config_renderer: i32,

    // Event queue
    pub event_counter: AtomicUsize,
    pub events: [GLFWInputEvent; EVENT_WINDOW_SIZE],
    pub out_event_index: usize,
    pub out_target_index: usize,
    pub in_event_index: usize,
    pub in_event_count: usize,

    // Cursor state
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub c_last_x: f64,
    pub c_last_y: f64,

    // JNI method/class pointers (using raw pointers to avoid lifetime issues)
    pub method_access_android_clipboard: *mut std::ffi::c_void,
    pub method_on_grab_state_changed: *mut std::ffi::c_void,
    pub method_on_cursor_shape_changed: *mut std::ffi::c_void,
    pub method_on_graphic_output: *mut std::ffi::c_void,
    pub method_glft_set_window_attrib: *mut std::ffi::c_void,
    pub method_internal_window_size_changed: *mut std::ffi::c_void,
    pub bridge_clazz: *mut std::ffi::c_void,
    pub vm_glfw_class: *mut std::ffi::c_void,

    // Input state
    pub is_grabbing: jboolean,
    pub key_down_buffer: *mut jbyte,
    pub mouse_down_buffer: *mut jbyte,

    // JVM pointers (raw pointers)
    pub runtime_java_vm_ptr: *mut libc::c_void,
    pub runtime_jni_env_ptr_jre: *mut std::ffi::c_void,
    pub dalvik_java_vm_ptr: *mut libc::c_void,
    pub dalvik_jni_env_ptr_android: *mut std::ffi::c_void,

    // Window state
    pub showing_window: jlong,

    // Flags
    pub is_input_ready: bool,
    pub is_cursor_entered: bool,
    pub is_use_stack_queue_call: bool,
    pub should_update_mouse: bool,
    pub has_graphic_output: bool,

    // Dimensions
    pub saved_width: i32,
    pub saved_height: i32,

    // GLFW callbacks
    pub glfw_invoke_char: GLFW_invoke_Char_func,
    pub glfw_invoke_char_mods: GLFW_invoke_CharMods_func,
    pub glfw_invoke_cursor_enter: GLFW_invoke_CursorEnter_func,
    pub glfw_invoke_cursor_pos: GLFW_invoke_CursorPos_func,
    pub glfw_invoke_framebuffer_size: GLFW_invoke_FramebufferSize_func,
    pub glfw_invoke_key: GLFW_invoke_Key_func,
    pub glfw_invoke_mouse_button: GLFW_invoke_MouseButton_func,
    pub glfw_invoke_scroll: GLFW_invoke_Scroll_func,
    pub glfw_invoke_window_size: GLFW_invoke_WindowSize_func,
}

/// Initialize environment
pub fn init_environ() {
    unsafe {
        let environ = Box::new(std::mem::zeroed::<PojavEnviron>());
        let boxed = Box::into_raw(environ);
        POJAV_ENVIRON.store(boxed, Ordering::SeqCst);
    }
}

/// Get pointer to environment
#[allow(static_mut_refs)]
pub fn get_environ() -> &'static mut PojavEnviron {
    let ptr = POJAV_ENVIRON.load(Ordering::SeqCst);
    if ptr.is_null() {
        init_environ();
    }
    // Return mutable reference
    unsafe { &mut *POJAV_ENVIRON.load(Ordering::SeqCst) }
}

// Getters and setters for common fields

pub fn get_pojav_window() -> *mut libc::c_void {
    get_environ().pojav_window
}

pub fn set_pojav_window(window: *mut libc::c_void) {
    get_environ().pojav_window = window;
}

pub fn get_renderer() -> i32 {
    get_environ().config_renderer
}

pub fn set_renderer(renderer: i32) {
    get_environ().config_renderer = renderer;
}

pub fn get_dalvik_java_vm() -> *mut libc::c_void {
    get_environ().dalvik_java_vm_ptr
}

pub fn set_dalvik_java_vm(vm: *mut libc::c_void) {
    get_environ().dalvik_java_vm_ptr = vm;
}

pub fn get_runtime_java_vm() -> *mut libc::c_void {
    get_environ().runtime_java_vm_ptr
}

pub fn set_runtime_java_vm(vm: *mut libc::c_void) {
    get_environ().runtime_java_vm_ptr = vm;
}

pub fn has_graphic_output() -> bool {
    get_environ().has_graphic_output
}

pub fn set_has_graphic_output(value: bool) {
    get_environ().has_graphic_output = value;
}

/// Initialize environ
#[no_mangle]
pub unsafe extern "C" fn Java_com_movtery_zalithlauncher_bridge_ZLBridge_initEnviron(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
) -> jint {
    init_environ();
    0
}
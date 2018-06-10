use std::{ptr, mem};
use x11::xlib::{
    Display,
    Window,
    XCloseDisplay,
    XDefaultScreen,
    XDestroyWindow,
    XOpenDisplay,
    XRootWindow,
    XGCValues,
    XCreateGC,
    GC,
    GXinvert,
    GCFunction,
    XFreeGC
};

pub struct XScreen {
    pub display: *mut Display,
    pub window_root: Window,
    pub gc: GC,
}

impl XScreen {
    pub fn new() -> XScreen {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            let screen = XDefaultScreen(display);
            let window_root = XRootWindow(display, screen);
            let gc = XScreen::create_gc(display, window_root.clone());
            XScreen { display, window_root, gc }
        }
    }

    fn create_gc(display: *mut Display, window: Window) -> GC {
        unsafe {
            let mut gc_values: XGCValues = mem::zeroed();
            gc_values.function = GXinvert;

            XCreateGC(
                display,
                window,
                GCFunction.into(),
                &mut gc_values,
            )
        }
    }
}

impl Drop for XScreen {
    fn drop(&mut self) {
        unsafe {
            XDestroyWindow(self.display, self.window_root);
            XCloseDisplay(self.display);
            XFreeGC(self.display, self.gc);
        }
    }
}
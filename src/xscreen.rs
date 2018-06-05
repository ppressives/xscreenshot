use image::{ImageBuffer, Rgb};
use std::ptr;
use x11::xlib::{
    Display,
    Window,
    XCloseDisplay,
    XDefaultScreen,
    XDestroyWindow,
    XOpenDisplay,
    XRootWindow,
};

pub struct XScreen {
    pub display: *mut Display,
    pub window_root: Window,
}

impl XScreen {
    pub fn new() -> XScreen {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            let screen = XDefaultScreen(display);
            let window_root = XRootWindow(display, screen);
            XScreen { display, window_root }
        }
    }
}

impl Drop for XScreen {
    fn drop(&mut self) {
        unsafe {
            XDestroyWindow(self.display, self.window_root);
            XCloseDisplay(self.display);
        }
    }
}
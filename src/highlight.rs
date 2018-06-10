use xscreen::XScreen;
use x11::xlib::{
    XAllowEvents,
    AsyncBoth,
    CurrentTime,
    XGrabPointer,
    ButtonPressMask,
    ButtonReleaseMask,
    GrabModeAsync,
    XEvent,
    XNextEvent,
    ButtonPress,
    ButtonRelease,
};
use std::mem;

pub struct Selection {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

struct Point(u32, u32);

impl XScreen {
    pub fn select_frame(&self) -> Selection {
        self.subscribe_events();

        let mut top_left: Point = Point(0, 0);
        let mut bottom_right: Point = Point(0, 0);

        unsafe {
            let mut event: XEvent = mem::zeroed();
            loop {
                XNextEvent(self.display, &mut event);
                match event.get_type() {
                    ButtonPress => {
                        top_left = Point(event.motion.x as u32, event.motion.y as u32);
                    }
                    ButtonRelease => {
                        bottom_right = Point(event.motion.x as u32, event.motion.y as u32);
                        break;
                    }
                    _ => ()
                }
            }
            Selection::new(top_left, bottom_right)
        }
    }

    fn subscribe_events(&self) {
        unsafe {
            XAllowEvents(self.display, AsyncBoth, CurrentTime);
            XGrabPointer(
                self.display,
                self.window_root,
                1,
                (ButtonPressMask | ButtonReleaseMask) as u32,
                GrabModeAsync,
                GrabModeAsync,
                0,
                0,
                CurrentTime,
            );
        }
    }
}

impl Selection {
    fn new(top_left: Point, bottom_right: Point) -> Selection {
        Selection { x1: top_left.0, y1: top_left.1, x2: bottom_right.0, y2: bottom_right.1 }
    }
}
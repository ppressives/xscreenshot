use xscreen::XScreen;
use selection::Selection;
use selection::Point;
use x11::xlib::{
    XAllowEvents,
    XGrabPointer,
    XDrawRectangle,
    AsyncBoth,
    CurrentTime,
    ButtonPressMask,
    ButtonReleaseMask,
    ButtonMotionMask,
    GrabModeAsync,
    XEvent,
    XNextEvent,
    ButtonPress,
    ButtonRelease,
    MotionNotify,
};
use std::mem;

impl XScreen {
    pub fn select_frame(&self) -> Selection {
        self.subscribe_events();

        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);

        let mut drawn = false;

        unsafe {
            let mut event: XEvent = mem::zeroed();
            loop {
                XNextEvent(self.display, &mut event);
                if drawn { //clear rect
                    self.draw_rect(Selection::new(&start, &end));
                }
                match event.get_type() {
                    ButtonPress => {
                        start = Point::new(event.motion.x as u32, event.motion.y as u32);
                        end = start.clone();
                    }
                    ButtonRelease => {
                        end = Point::new(event.motion.x as u32, event.motion.y as u32);
                        break;
                    }
                    MotionNotify => {
                        end = Point::new(event.motion.x as u32, event.motion.y as u32);
                    }
                    _ => ()
                }
                self.draw_rect(Selection::new(&start, &end));
                drawn = true;
            }

            Selection::new(&start, &end)
        }
    }

    fn subscribe_events(&self) {
        unsafe {
            XAllowEvents(self.display, AsyncBoth, CurrentTime);
            XGrabPointer(
                self.display,
                self.window_root,
                1,
                (ButtonMotionMask | ButtonPressMask | ButtonReleaseMask) as u32,
                GrabModeAsync,
                GrabModeAsync,
                0,
                0,
                CurrentTime,
            );
        }
    }

    fn draw_rect(&self, selection: Selection) {
        unsafe {
            XDrawRectangle(
                self.display,
                self.window_root,
                self.gc,
                selection.x1 as i32,
                selection.y1 as i32,
                selection.width(),
                selection.height(),
            );
        }
    }
}
use x11::xlib;
use std::ptr;
use std::slice;

use image::{ImageBuffer, Rgb};

pub struct XScreen {
    display: *mut xlib::Display,
    window_root: xlib::Window,
}

/*
    XImage data in 32bpp is always blue, green, red and nul bytes.
*/
struct BGR(u8, u8, u8, u8);

impl XScreen {
    pub fn new() -> XScreen {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let screen = xlib::XDefaultScreen(display);
            let window_root = xlib::XRootWindow(display, screen);
            XScreen { display, window_root }
        }
    }

    pub fn capture_frame(&self, x: i32, y: i32, w: u32, h: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let x_image;
        unsafe {
            x_image = xlib::XGetImage(
                self.display,
                self.window_root,
                x,
                y,
                w,
                h,
                xlib::XAllPlanes(),
                xlib::ZPixmap,
            );
        }
        let pixels = XScreen::image_to_pixels(x_image, (w * h) as usize);
        XScreen::create_image_buffer(pixels, w, h)
    }

    fn image_to_pixels<'a>(image: *mut xlib::XImage, size: usize) -> &'a [BGR] {
        unsafe {
            let ptr = (*image).data as *const BGR;
            slice::from_raw_parts(ptr, size * 4)
        }
    }

    fn create_image_buffer(pixels: &[BGR], w: u32, h: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(w, h);
        {
            let pairs = pixels.iter().zip(img.pixels_mut());
            for (image_pixel, buffer_pixel) in pairs {
                buffer_pixel.data = [image_pixel.2, image_pixel.1, image_pixel.0];
            }
        }
        img
    }
}
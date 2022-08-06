extern crate core;

pub mod audio;

use std::ops::{Deref, DerefMut};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct RetroCanvas {
    simulated_size : (u32, u32),
    interior: Canvas<Window>
}

impl Deref for RetroCanvas {
    type Target = Canvas<Window>;

    fn deref(&self) -> &Self::Target {
        &self.interior
    }
}

impl DerefMut for RetroCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.interior
    }
}

impl RetroCanvas {
    /// Creates a new RetroCanvas with the given real size, simulated size, and title.
    /// The simulated size is the size of your console's display, and is what you write to.
    /// The real size is the size of the window. If the ratios are different, parts of the real window will be whatever color you clear with, by default black.
    /// The title is the title of the window.
    /// If you wish to force integer scaling, call `set_integer_scaling(true)` after creating the canvas.
    pub fn new(real_size : (u32, u32), simulated_size : (u32, u32), title : &str) -> RetroCanvas {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(title, real_size.0, real_size.1)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = RetroCanvas {
            simulated_size,
            interior: window.into_canvas().build().unwrap()
        };
        canvas.set_logical_size(simulated_size.0, simulated_size.1).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        canvas
    }

    /// Clears the simulated screen of your console with whatever color you set.
    pub fn clear_simulated(&mut self) {
        self.interior.fill_rect(Rect::new(0, 0, self.simulated_size.0, self.simulated_size.1)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn scaling() {
        use super::*;
        let mut canvas = RetroCanvas::new((1000, 600), (320, 240), "Test");
        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.clear_simulated();
        canvas.present();
        std::thread::sleep(Duration::from_secs(5));
    }
}

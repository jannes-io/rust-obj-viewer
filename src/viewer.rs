extern crate sdl2;
use sdl2::pixels::Color;

use crate::object::*;

pub struct Viewer {
    object: Object,
    canvas: sdl2::render::WindowCanvas,
}

impl Viewer {
    pub fn new(obj: Object) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Object viewer", 1280, 720)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGB(21, 21, 21));
        canvas.clear();
        canvas.present();

        Ok(Self {
            object: obj,
            canvas: canvas
        })
    }

    pub fn run(&mut self) {
        
    }
}
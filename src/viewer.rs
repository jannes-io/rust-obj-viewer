extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod camera;
use crate::geometry::*;
use crate::object::*;
use camera::Camera;

impl From<Vec2> for sdl2::rect::Point {
    fn from(v2: Vec2) -> Self {
        sdl2::rect::Point::new(v2.x as i32, v2.y as i32)
    }
}

pub struct Viewer {
    object: Object,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    camera: Camera,
}

impl Viewer {
    pub fn new(obj: Object) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        const DIMENSIONS: Vec2 = Vec2 {
            x: 1280.0,
            y: 720.0,
        };
        let window = video_subsystem
            .window("Object viewer", DIMENSIONS.x as u32, DIMENSIONS.y as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            object: obj,
            canvas: canvas,
            event_pump: event_pump,
            camera: Camera::new(DIMENSIONS),
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            self.canvas.set_draw_color(Color::RGB(21, 21, 21));
            self.canvas.clear();
            self.draw_object().expect("Could not draw object");
            self.canvas.present();
        }
    }

    fn draw_object(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (v1, v2, v3) in &self.object.triangles {
            let p1 = self.camera.project_vec(&v1.pos);
            let p2 = self.camera.project_vec(&v2.pos);
            let p3 = self.camera.project_vec(&v3.pos);
            self.canvas.draw_line(p1, p2)?;
            self.canvas.draw_line(p2, p3)?;
            self.canvas.draw_line(p1, p3)?;
        }
        Ok(())
    }
}

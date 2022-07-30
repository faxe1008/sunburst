use super::canvas::Canvas;
use std::io::Write;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelMasks;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::EventPump;

pub enum RendererType<'a> {
    PPM(Box<dyn Write>),
    SDL2(&'a str),
}

pub trait Renderer {
    fn update(&mut self, canvas: &Canvas) -> bool;
}

pub struct PPMRenderer {
    writer: Box<dyn Write>,
}

impl PPMRenderer {
    pub fn new(writer: Box<dyn Write>) -> Self {
        PPMRenderer { writer }
    }
}

impl Renderer for PPMRenderer {
    fn update(&mut self, canvas: &Canvas) -> bool {
        writeln!(
            self.writer,
            "P6\n{} {}\n255",
            canvas.width(),
            canvas.height()
        )
        .unwrap();
        self.writer.write(canvas.as_raw_buffer()).unwrap();
        true
    }
}

pub struct SDLRenderer {
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    event_pump: EventPump,
}

impl SDLRenderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump().unwrap();

        SDLRenderer {
            canvas,
            texture_creator,
            event_pump,
        }
    }
}

impl Renderer for SDLRenderer {
    fn update(&mut self, canvas: &Canvas) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                _ => {}
            }
        }

        let surface = Surface::from_data_pixelmasks(
            canvas.as_raw_buffer_mut(),
            canvas.width() as u32,
            canvas.height() as u32,
            3 * canvas.width() as u32,
            PixelMasks {
                bpp: 24,
                rmask: 0x0000ff,
                gmask: 0x00ff00,
                bmask: 0xff0000,
                amask: 0,
            },
        )
        .unwrap();
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let rect = sdl2::rect::Rect::new(0, 0, canvas.width() as u32, canvas.height() as u32);
        self.canvas
            .copy_ex(&texture, Some(rect), Some(rect), 0.0, None, false, false)
            .unwrap();
        self.canvas.present();
        true
    }
}

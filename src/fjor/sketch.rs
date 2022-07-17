use super::{
    canvas::Canvas,
    renderer::{PPMRenderer, Renderer, RendererType, RendererType::PPM},
};

pub struct Sketch<'a> {
    canvas: Canvas,
    frame: usize,
    pub on_setup: &'a dyn Fn(&mut Canvas),
    pub on_update: &'a dyn Fn(&mut Canvas, usize),
    renderer: Box<dyn Renderer>,
}

impl<'a> Sketch<'a> {
    pub fn new(width: usize, height: usize, renderer_type: RendererType) -> Self {
        let renderer = match renderer_type {
            PPM(writer) => PPMRenderer::new(writer),
        };

        Sketch {
            canvas: Canvas::new(width, height),
            frame: 1,
            on_setup: &|_: &mut Canvas| {},
            on_update: &|_: &mut Canvas, _: usize| {},
            renderer: Box::new(renderer),
        }
    }

    pub fn run(&mut self) {
        (self.on_setup)(&mut self.canvas);
        loop {
            (self.on_update)(&mut self.canvas, self.frame);
            self.renderer.show(&&self.canvas);
            self.frame = self.frame.wrapping_add(1);
            self.canvas.clear();
        }
    }
}

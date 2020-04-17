
use sdl2::{
  
    render::Canvas,
    video::Window,
    pixels::PixelFormatEnum,

};
use sdl2::Sdl;

use crate::framebuffer::FrameBuffer;

pub struct SdlHelper{
     sdl_context:Sdl
}

impl SdlHelper{
    pub fn new ()->Self {
        let sdl_context = sdl2::init().unwrap();
        SdlHelper{
            sdl_context
        }
    }

pub fn sdl_context(&self)-> &Sdl{
&self.sdl_context
}

pub fn create_canvas (&self,width:u32 , height:u32 , title:&str)-> Canvas<Window>{
    let video_ctx =self.sdl_context.video().unwrap();

    let window = match video_ctx.window(
        title,
        width,
        height)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err)   => panic!("failed to create renderer: {}", err)
    };

    return canvas;
}


pub fn put_framebuffer_in_canvas(&self,canvas:&mut Canvas<Window> , frame:&FrameBuffer){

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, frame.width as u32, frame.height as u32).unwrap();
        
    // Create a red-green gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..frame.height {
            for x in 0..frame.width {
                let offset = y*pitch + x*4;
                let pixel =frame.get_pixel(x, y).unwrap();
                buffer[offset] = pixel.a;
                buffer[offset + 1] = pixel.b;
                buffer[offset + 2] = pixel.g;
                buffer[offset + 3] = pixel.r;
            }
        }
    }).unwrap();

    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

}

}
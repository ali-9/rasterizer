
extern crate rasterizer ;
extern crate rand;

use rasterizer::{ cube::* ,transformers::*, defs::* ,renderer::* ,  input::*} ;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Instant;
use cgmath::*;
use rand::Rng;


struct UserGame {
renderer:Renderer,
input:Input,

delta: u128  ,
theta_z:f32 ,
random_colors :Vec<Color>,
cube :Cube
}

impl UserGame {
    pub fn new(screen_width:u32,
        screen_height:u32)-> Self{

    let mut rng = rand::thread_rng();
  
    let mut random_colors: Vec<Color> = Vec::with_capacity(12); 

    for _ in 0..12{
        random_colors.push(Color::RGB(rng.gen(),rng.gen(),rng.gen()));
    }

    let  renderer =Renderer::new(screen_width, screen_height, "raster");
    let input = Input::new();
    let cube :Cube = Cube::new(0.5);

    UserGame{
        renderer,
        input,
        delta:0,
        theta_z:0.0,
        random_colors,
        cube
    }
    }

   pub fn run(&mut self){

    let sdl_context = self.renderer.get_sdl_context().sdl_context();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'game_loop: loop {
     
        if self.input.key_pressed(&mut event_pump,Keycode::Escape) || self.input.is_quit(& mut event_pump) {
            break 'game_loop;
        }
        
        if self.input.key_pressed(&mut event_pump, Keycode::A){
        println!("yoooooooooooo" );
        }

self.update();
self.render();
    }
}

  
fn update(&self) {
      


            


}

    fn render(&mut self) {
       
    
            let before = Instant::now();
            self.renderer.clear();
            let cube_buffer = self.cube.get_indexed_buffer();
           
    
    
            let mut transformed_vertices:Vec<Vec3f>= Vec::with_capacity(cube_buffer.vertices.len());
            self.theta_z=wrap_angle(self.theta_z+ std::f32::consts::PI/60.0);
          //  Matrix4::from(ortho(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
            let rot:Matrix4<f32>=Matrix4::from_angle_x(Rad(self.theta_z))
            *Matrix4::from_angle_y(Rad(self.theta_z))
            *Matrix4::from_angle_z(Rad(self.theta_z));
            for v in cube_buffer.vertices.iter() {
                 let mut  transformed_v  =*v;
                 transformed_v = rot.transform_vector(transformed_v);
                    transformed_v.z+=1.0;
                let  sp_v= ndc_to_screen_space(&transformed_v, self.renderer.get_size().x as u32 , self.renderer.get_size().y as u32 );
                transformed_vertices.push(sp_v);
            }
    
            let mut indecies_iter = cube_buffer.indices.iter();
            let mut i=0;
            loop{
              match indecies_iter.next(){
                  Some(index1)=>{
                   let index2= indecies_iter.next().unwrap();
                   let index3= indecies_iter.next().unwrap();
                   let v0 = transformed_vertices[*index1];
                   let v1 = transformed_vertices[*index2];
                   let v2 = transformed_vertices[*index3];
                  // println!("{:?},{:?}",start,end);
                  self. renderer.draw_trangle(&v0.xy(), &v1.xy(), &v2.xy(), self.random_colors[i]);
                   i+=1;
                   //renderer.draw_line(&start.xy(), &end.xy(), Color::RGB(255, 255, 255));
                },
                  None=>{break;}
              }
    
            }   
            
            
          
           self. renderer.present();
    
            let after = before.elapsed();
           self. delta += after.as_millis();
            if self.delta >= 1000 {
                self.delta = 0;
                println!(
                    "Ticks:{:.2?} / fps:{:?}",
                    after,
                    1000.0 as u128 / after.as_millis()
                );
            }
       
    }
  

}




pub fn main() {
let mut g = UserGame::new(620, 620);
g.run();
}


pub fn rot<R: Rotation3<f32>>(deg:f32) -> R {
    let axis = Vector3::new(0.0, 0.0, 1.0).normalize();
    Rotation3::from_axis_angle(axis, Deg(deg))
}
extern crate sdl2;

use rand::prelude::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use std::{thread, time};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const POINTS: usize = 32;

const SIXTEEN_M: time::Duration = time::Duration::new(0, 16000000);

fn draw_chaos(loc: [(f32, f32, f32); POINTS], canvas: &mut Canvas<Window>) -> Result<(), String> {
    let rects = loc.into_iter().filter_map(|(x, y, _)| -> Option<Rect> {
        Some(Rect::new(
            x.round() as i32 * 2 + (WINDOW_WIDTH as i32 / 2),
            y.round() as i32 * 2 + (WINDOW_HEIGHT as i32 / 2),
            2,
            2,
        ))
    });
    let rects: Vec<Rect> = rects.collect();

    canvas.draw_rects(&rects)?;
    Ok(())
    //Draw rainbow string behind based on distance
}

struct Slider<'a> {
    text: String,
    sw: u32,
    sh: u32,
    bw: u32,
    bh: u32,
    x: i32,
    y: i32,
    value: &'a mut f32,
}

impl<'a> Slider<'a> {
    fn new(
        text: String,
        sw: u32,
        sh: u32,
        bw: u32,
        bh: u32,
        x: i32,
        y: i32,
        value: &'a mut f32,
    ) -> Slider {
        Slider {
            text,
            sw,
            sh,
            bw,
            bh,
            x,
            y,
            value,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(self.x, self.y, self.sw, self.sh);
        let bar = Rect::new(
            self.x + (*self.value) as i32,
            self.y,
            self.bw,
            self.bh,
        );
        canvas.fill_rect(rect)?;
        canvas.fill_rect(bar)?;
        //canvas.draw_line(self.x, (self.position.0 + self.size as i32 * 100, self.position.1))?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Lorenz", WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let loc: [(f32, f32, f32); POINTS] = random();

    let mut loc: [(f32, f32, f32); POINTS] = loc.map(|(x, y, z)| (x * 10.0, y * 10.0, z * 10.0));
    draw_chaos(loc, &mut canvas)?;

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut sigma = 15_f32;
    let mut beta = 13_f32 / 3_f32;
    let mut rho = 52_f32;

    let sigma_button = Slider::new(String::from("Sigma"), 100, 4, 2, 10, 15, 15, &mut sigma);
    let beta_button = Slider::new(String::from("Sigma"), 100, 4, 2, 10, 15, 65, &mut beta);
    let rho_button = Slider::new(String::from("Sigma"), 100, 4, 2, 10, 15, 115, &mut rho);
    

    'running: loop {
        let now = time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion { mousestate, x, y, .. } => {
                    if mousestate.left() {
                        match (x, y) {
                            (15..=115, 15..=35) => {
                                *sigma_button.value = (x - sigma_button.x) as f32;
                            },
                            (15..=115, 65..=85) => {
                                *beta_button.value = (x - sigma_button.x) as f32;
                            },
                            (15..=115, 115..=135) => {
                                *rho_button.value = (x - sigma_button.x) as f32;
                            },
                            _ => ()
                        }
                    } 
                },
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            }
        }
        
    

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_chaos(loc, &mut canvas)?;

        sigma_button.draw(&mut canvas)?;
        rho_button.draw(&mut canvas)?;
        beta_button.draw(&mut canvas)?;

        loc = loc.map(|(mut x, mut y, mut z)| {
            x += (*sigma_button.value * (y - x)) * 0.01;
            y += (x * (*rho_button.value - z) - y) * 0.01;
            z += ((x * y) - (*beta_button.value * z)) * 0.01;
            (x, y, z)
        });
        
        println!("S: {}, R: {}, B: {}", *sigma_button.value, *rho_button.value, *beta_button.value);
        
        canvas.set_draw_color(Color::RGB(100, 255, 2));
        draw_chaos(loc, &mut canvas)?;

        //println!("x: {}, y: {}", x, y);
        canvas.present();

        if now.elapsed() < SIXTEEN_M {
            thread::sleep(SIXTEEN_M - now.elapsed())
        }
    }

    Ok(())
}

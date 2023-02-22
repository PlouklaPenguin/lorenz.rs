extern crate sdl2;

use rand::prelude::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use std::{thread, time};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const POINTS: usize = 18;

const SIXTEEN_M: time::Duration = time::Duration::new(0, 16000000);

fn draw(loc: [(f32, f32, f32); POINTS], canvas: &mut Canvas<Window>) -> Result<(), String> {
    let rects = loc.into_iter().filter_map(|(x, _, y)| -> Option<Rect> {
        if x.round() as u32 > WINDOW_WIDTH && y.round() as u32 > WINDOW_HEIGHT {
            None
        } else {
            Some(Rect::new(
                x.round() as i32 + (WINDOW_WIDTH as i32 / 2),
                y.round() as i32 + (WINDOW_HEIGHT as i32 / 2),
                2,
                2,
            ))
        }
    });
    let rects: Vec<Rect> = rects.collect();

    canvas.draw_rects(&rects)?;
    Ok(())
    //Draw rainbow string behind based on distance
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
    /*[
        (1.0, 2.0, 1.0),
        (2.0, 3.0, 4.0),
        (1.0, 2.0, 5.0),
        (3.0, 4.0, 6.0),
    ];*/

    let mut loc: [(f32, f32, f32); POINTS] = loc.map(|(x, y, z)| (x * 10.0, y * 10.0, z * 10.0));
    draw(loc, &mut canvas)?;

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let sigma = 10_f32;
    let beta = 8_f32 / 3_f32;
    let rho = 15_f32;

    'running: loop {
        let now = time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            }
        }

        loc = loc.map(|(mut x, mut y, mut z)| {
            x += (sigma * (y - x)) * 0.02;
            y += (x * (rho - z) - y) * 0.02;
            z += ((x * y) - (beta * z)) * 0.02;
            (x, y, z)
        });

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw(loc, &mut canvas)?;

        //println!("x: {}, y: {}", x, y);
        canvas.present();

        if now.elapsed() < SIXTEEN_M {
            thread::sleep(SIXTEEN_M - now.elapsed())
        }
    }

    Ok(())
}

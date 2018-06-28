extern crate chip16;
extern crate sdl2;

use chip16::{Cpu, Rom};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::fs::File;
use std::time::Duration;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let rom = Rom::new(file).unwrap();

    let cpu = Cpu::new();
    // cpu.load(rom);

    run();
}

fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("chip16_sdl2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

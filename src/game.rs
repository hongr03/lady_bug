use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::render::WindowCanvas;

pub struct Game {
    sc: Sdl,
    cvs: WindowCanvas,
    t: Instant,
    run: bool,
}

impl Game {
    const WIN_WIDTH: u32 = 800;
    const WIN_HEIGHT: u32 = 600;

    pub fn new() -> Self {
        let sc = sdl2::init().unwrap();
        let vs = sc.video().unwrap();
        let win = vs.window("Hello World", Self::WIN_WIDTH, Self::WIN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let cvs = win.into_canvas().build().unwrap();
        let t = Instant::now();
        let run = true;
        Game {sc, cvs, t, run}
    }

    pub fn run(&mut self) {
        while self.run {
            self.input();
            self.update();
            self.output();
        }
    }

    pub fn quit(&mut self) {
        
    }

    fn input(&mut self) {
        let mut ep = self.sc.event_pump().unwrap();
        for e in ep.poll_iter() {
            match e {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.run = false;
                }
                _ => {}
            }
        }
    }

    fn sleep(&self) {
        let fps = Duration::new(0, 1e9 as u32 / 60);
        let gap = Instant::now() - self.t;
        if gap > fps {
            std::thread::sleep(gap - fps);
        }
    }

    fn update(&mut self) {
        self.sleep();
        let mut dt = (Instant::now() - self.t).as_secs_f32();
        self.t = Instant::now();
        if dt > 0.05 {
            dt = 0.05;
        }
    }

    fn output(&mut self) {

    }
}

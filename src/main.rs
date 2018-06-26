extern crate sdl2;

use std::process;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

mod board;
mod cell;

fn main() {
  let ctx = sdl2::init().unwrap();
  let video_ctx = ctx.video().unwrap();
  let width = 640;
  let height = 480;

  let board = board::Board {
      height: 640,
      width: 480,
  };

  let window  = match video_ctx
      .window("Game of Rust", width, height)
      .position_centered()
      .opengl()
      .build() {
          Ok(window) => window,
          Err(err)   => panic!("failed to create window: {}", err)
      };

  let mut renderer = match window
      .into_canvas()
      .build() {
          Ok(renderer) => renderer,
          Err(err) => panic!("failed to create renderer: {}", err)
      };

  let rect = Rect::new(10, 10, 10, 10);
  let black = sdl2::pixels::Color::RGB(0, 0, 0);
  let white = sdl2::pixels::Color::RGB(255, 255, 255);

  let mut events = ctx.event_pump().unwrap();

  let mut main_loop = || {
    for event in events.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                process::exit(1);
            },
            _ => {}
        }
    }

      let _ = renderer.set_draw_color(black);
      let _ = renderer.clear();
      let _ = renderer.set_draw_color(white);
      let _ = renderer.fill_rect(rect);
      let _ = renderer.present();
  };

  loop { main_loop(); }
}
extern crate alsa;
extern crate rust_gpiozero;

pub mod sound;
pub mod graphics;
pub mod screens;

use rand::Rng;
use std::sync::mpsc;
use alsa::mixer::{Mixer, SelemChannelId};
use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions};
use std::{thread, time};

fn main() {
    let mixer = match Mixer::new("hw:0", false) {
        Ok(mixer) => mixer,
        Err(err) => panic!(err),
    };
    let speaker = sound::Speaker::new(&mixer, "Speaker", SelemChannelId::FrontLeft);

    if !speaker.test() {
        panic!("Speaker not working correctly.");
    }

    print!("Listening for input.");
    
    let render_delay = time::Duration::from_millis(1000/60);
    let color = &LedColor {
        red: 255, blue: 0, green: 0,
    };

    // let (tx, rx) = mpsc::channel();

    let mut config = LedMatrixOptions::new();
    config.set_rows(16);
    let matrix = match LedMatrix::new(Some(config)) {
        Ok(m) => m,
        Err(err) => panic!(err),
    };
    let mut canvas = matrix.canvas();
    let mut rng = rand::thread_rng();
    let mut state = "menu";
    screens::get(state);

    loop {
        let instructions = screens::get(state)();
        canvas.clear();
        // canvas.set(rng.gen_range(0, 30), rng.gen_range(0, 10), color);
        for inst in &instructions {
            for pixel in &inst.pixels {
                let (x, y, color) = pixel;
                canvas.set(*x, *y, color);
            }
        }
        thread::sleep(render_delay);
    }

    // let color = &LedColor {
    //     red: 0, blue: 255, green: 0,
    // };
}

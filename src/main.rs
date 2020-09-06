extern crate alsa;
extern crate rust_gpiozero;

pub mod sound;
pub mod graphics;

use rand::Rng;
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
    let mut config = LedMatrixOptions::new();
    config.set_rows(16);


    let matrix = match LedMatrix::new(Some(config)) {
        Ok(m) => m,
        Err(err) => panic!(err),
    };
    let mut canvas = matrix.canvas();
    let mut rng = rand::thread_rng();
    
    
    let render_delay = time::Duration::from_millis(1000/60);

    loop {
        canvas.clear();
        canvas.set(rng.gen_range(0, 30), rng.gen_range(0, 10), &LedColor {
            red: 255, blue: 0, green: 0,
        });
        thread::sleep(render_delay);
    }

}

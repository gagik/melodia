extern crate alsa;
extern crate rppal;

pub mod sound;
pub mod graphics;
pub mod screens;

use rand::Rng;
use std::sync::mpsc;
use alsa::mixer::{Mixer, SelemChannelId};
use rpi_led_matrix::{LedColor, LedCanvas, LedMatrix, LedMatrixOptions};
use screens::{InstructionType, Instruction, Canvas, rect};
use rppal::gpio::{Gpio};
use std::{thread, time};

fn main() {
    let mixer = match Mixer::new("hw:0", false) {
        Ok(mixer) => mixer,
        Err(err) => panic!(err),
    };
    let mut gpio = Gpio::new().unwrap();
    let speaker = sound::Speaker::new(&mixer, "Speaker", SelemChannelId::FrontLeft);

    if !speaker.test() {
        panic!("Speaker not working correctly.");
    }
    println!("Speaker Volume: {}%", speaker.get_volume());


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
    let mut rng = rand::thread_rng();
    let mut state = "menu";
    screens::get(state);

    let mut soundKnob = sound::Knob::new(gpio, 20, 21);

    let mut overlay = Vec::new();
    let mut canvas = Canvas::new(matrix, 32, 16, 0, 0);
    loop {
        let instructions = screens::get(state)();
        let i = soundKnob.update();

        if i > 0 {
            speaker.set_volume(speaker.get_volume()+1);
            let progress = speaker.get_volume_percent()/100.0 * 30.0;
            overlay.push(screens::rect(2,2, progress as i32, 3, 0, 100, 100));
        } else if i < 0 {
            speaker.set_volume(speaker.get_volume()-1);
            let progress = speaker.get_volume_percent()/100.0 * 30.0;
            overlay.push(screens::rect(2,2, progress as i32, 3, 0, 100, 100));
        }

        canvas.runInstructions(&overlay);
        canvas.runInstructions(&instructions);
        
        thread::sleep(render_delay);
    }
}

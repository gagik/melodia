#![allow(non_snake_case)]

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
use std::collections::HashMap;
use std::cell::RefCell;

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

    
    let mut overlay = HashMap::new();
    let progress = speaker.get_volume_percent()/100.0 * 30.0;

    let mut canvas = matrix.canvas();
    loop {
        canvas.clear();
        let instructions = screens::get(state)();
        let i = soundKnob.update();

        if i > 0 {
            speaker.set_volume(speaker.get_volume()+1);
            let progress = speaker.get_volume_percent()/100.0 * 28.0;
            let mut rect = screens::rect(2,5, progress as i32, 3, 0, 100, 100);
            rect.duration = 50;
            overlay.insert("volume", RefCell::new(rect));
        } else if i < 0 {
            speaker.set_volume(speaker.get_volume()-1);
            let progress = speaker.get_volume_percent()/100.0 * 28.0;
            let mut rect = screens::rect(2,5, progress as i32, 3, 0, 100, 100);
            rect.duration = 50;
            overlay.insert("volume", RefCell::new(rect));
        }

        for inst in &instructions 
        {
            match &inst.category {
                InstructionType::Line => {                    
                    canvas.draw_line(inst.x, inst.y, inst.y + inst.width-1, inst.y + inst.height-1, &inst.color)
                },
                InstructionType::Circle => {                    
                    canvas.draw_circle(inst.x, inst.y, inst.width as u32 /2 , &inst.color)
                },
                InstructionType::Rectangle => {
                    for i in inst.y..inst.y+inst.height {
                        canvas.draw_line(inst.x, i, inst.x+inst.width-1, i, &inst.color)
                    }
                },
                _ => {
                    for pixel in &inst.pixels {
                        let (x, y, color) = pixel;
                        canvas.set(*x, *y, color);
                    }
                }
            }
        }
        for (name, instCell) in &overlay 
        {
            let mut inst = instCell.borrow_mut();
            if inst.hidden {
                continue;
            }
            if inst.duration != -1 {
                let dur = inst.duration;
                inst.setDuration(dur-1);
                if  inst.duration == 0 {
                    inst.hidden = true;
                    continue;
                }
            }
            match &inst.category {
                InstructionType::Line => {                    
                    canvas.draw_line(inst.x, inst.y, inst.y + inst.width-1, inst.y + inst.height-1, &inst.color)
                },
                InstructionType::Circle => {                    
                    canvas.draw_circle(inst.x, inst.y, inst.width as u32 /2 , &inst.color)
                },
                InstructionType::Rectangle => {
                    for i in inst.y..inst.y+inst.height {
                        canvas.draw_line(inst.x, i, inst.x+inst.width-1, i, &inst.color)
                    }
                },
                _ => {
                    for pixel in &inst.pixels {
                        let (x, y, color) = pixel;
                        canvas.set(*x, *y, color);
                    }
                }
            }
        }
        
        thread::sleep(render_delay);
    }
}

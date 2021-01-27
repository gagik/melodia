#![allow(non_snake_case)]
use rpi_led_matrix::{LedColor, LedCanvas, LedMatrix};

pub enum InstructionType {
    Unset,
    Rectangle,
    Line,
    Circle
}

pub struct Instruction {
    pub pixels: Vec<(i32, i32, LedColor)>,
    pub category: InstructionType,
    pub value: &'static str,
    pub x:i32,
    pub y:i32,
    pub color: LedColor,
    pub width: i32,
    pub height: i32,
    pub hidden: bool,
    pub duration: i32,
}

impl Instruction {
    pub fn setDuration(&mut self, d:i32) {
        self.duration = d;
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            pixels: Vec::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            category: InstructionType::Unset,
            hidden: false,
            color: LedColor{red: 0, green:100, blue: 100},
            value: "",
            duration: -1,
        }
    }
}

pub struct Canvas {
    width:u8,
    height:u8,
    offset_x:u8,
    offset_y:u8,
    c: LedCanvas,
}

// impl Canvas {
//     pub fn new(matrix:LedMatrix, width:u8, height:u8, offset_x:u8, offset_y:u8) -> Self {
//         Canvas {
//             width,
//             height,
//             offset_x,
//             offset_y,
//             c: matrix.canvas(),
//         }
//     }
// }


fn pixel(x:i32, y:i32, r:u8, g:u8, b:u8) -> (i32, i32, LedColor) {
    (
        x, y, LedColor{
            red: r, green: g, blue: b
        }
    )
}


pub fn rect(x:i32, y:i32, w:i32, h:i32, r:u8, g:u8, b:u8) -> Instruction {
    Instruction{
        x,
        y,
        width: w,
        height: h,
        color: LedColor{red: r, green: g, blue: b},
        category: InstructionType::Rectangle,
        ..Instruction::default()
    }
}

fn line(x1:i32, y1:i32, x2:i32, y2:i32, r:u8, g:u8, b:u8) -> Instruction {
    Instruction{
        x: x1,
        y: y1,
        width: x2 - x1,
        height: y2 - y1,
        color: LedColor{red: r, green: g, blue: b},
        category: InstructionType::Line,
        ..Instruction::default()
    }
}

fn circle(x1:i32, y1:i32, radius:i32, r:u8, g:u8, b:u8) -> Instruction {
    Instruction{
        x: x1,
        y: y1,
        width: 2 * radius,
        height: 2 * radius,
        color: LedColor{red: r, green: g, blue: b},
        category: InstructionType::Circle,
        ..Instruction::default()
    }
}

fn menu() -> Vec<Instruction> {
    let mut actions = Vec::new();
    // let mut test = Instruction::default();
    // test.pixels.push(pixel(5,5,255,0,0));
    // test.pixels.push(pixel(5,6,255,0,0));
    // actions.push(test);
    // actions.push(rect(10, 5, 5, 5, 0, 255, 0));
    // actions.push(line(0, 0, 32, 16, 200, 200, 100));
    // actions.push(circle(5, 5, 3, 0, 255, 255));
    actions
}

pub fn get(state:&str) -> fn() -> Vec<Instruction> {
    match state {
        "menu" => {
            return menu
        }
        _ => return menu
    }
}
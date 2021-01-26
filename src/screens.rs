use rpi_led_matrix::{LedColor};

enum InstructionType {
    Unset,
}

fn pixel(x:i32, y:i32, r:u8, g:u8, b:u8) -> (i32, i32, LedColor) {
    (
        x, y, LedColor{
            red: r, green: g, blue: b
        }
    )
}

pub struct Instruction {
    pub pixels: Vec<(i32, i32, LedColor)>,
    category: InstructionType,
    x:i32,
    y:i32,
    width: i32,
    height: i32,
    hidden: bool,
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
        }
    }
}


fn menu() -> Vec<Instruction> {
    let mut actions = Vec::new();
    let mut test = Instruction::default();
    test.pixels.push(pixel(5,5,255,0,0));
    actions.push(test);
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
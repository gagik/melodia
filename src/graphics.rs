use rpi_led_matrix::{LedCanvas, LedColor};

trait Renderer {
    fn render(&mut self) -> ();
}


pub struct Matrix {
    width:u8,
    height:u8,
    offset_x: u8,
    offset_y: u8,
    canvas: LedCanvas,
    layers: Vec<Element>,
}

impl Matrix {
    pub fn new(canvas:LedCanvas, width:u8, height:u8, offset_x:u8, offset_y:u8) -> Self {
        Matrix {
            width,
            height,
            offset_x,
            offset_y,
            canvas,
            layers: Vec::new(),
        }
    }
}

enum ElementType {
    Unset,
}

pub struct Position {
    x:u8,
    y:u8,
}

pub struct Size {
    width:u8,
    height:u8,
}

pub struct Element {
    width:u8,
    height:u8,
    x:u8,
    y:u8,
    anchor_x:u8,
    anchor_y:u8,
    hidden:bool,
    category: Option<ElementType>,
    matrix: &'static Matrix,
}

// impl Default for Element {
//     fn default() -> Self {
//         Element {
//             x: 0,
//             y: 0,
//             width: 0,
//             height: 0,
//             anchor_x: 0,
//             anchor_y: 0,
//             type: Graphics
//             hidden: false,
//             matrix: None,
//         }
//     }
// }


// TODO: consider using defaults.
impl Element {
    pub fn new(position:Option<Position>, matrix:&'static Matrix) -> Self {
        let position = if let Some(pos_val) = position {pos_val } 
                       else { Position {x:0, y:0} };

        Element {
            x: position.x, 
            y: position.y, 
            matrix,
            width: 0,
            height: 0,
            anchor_x: 0,
            anchor_y: 0,
            hidden: false,
            category: None,
        }
    }
    pub fn hide(&mut self) { self.hidden = true; }
    pub fn show(&mut self) { self.hidden = false; }
}

pub struct Rectangle {
    el:Element,
}

// TODO: make a decision whether to use options or not.
// TODO: consistency between width/height vs size.
impl Rectangle {
    pub fn new(position:Position, size:Size, matrix:&'static Matrix) -> Self {
        let mut el = Element::new(Some(position), matrix);
        el.width = size.width;
        el.height = size.height;
        Rectangle {
            el
        }
    }
}

impl Renderer for Rectangle {
    fn render(&mut self) -> () {

    }
}
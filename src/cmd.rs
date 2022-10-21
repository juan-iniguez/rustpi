
use core::str::from_utf8;
use core::str::from_utf8_mut;

use crate::fb::*;
use crate::io::uart_writeHex;
use crate::mb::*;
use crate::terminal::FONT_BPG;
use crate::terminal::FONT_HEIGHT;
use crate::terminal::FONT_WIDTH;

static mut t_ptr:(u32,u32) = (10, 10);

pub fn cmd_init () {
    // Check if FB is initialized
    if unsafe {fb as u32 != 0} {
        // Start the Pointer for the Line 
        let txt_ptr = unsafe{&t_ptr};
        setPtr(&txt_ptr.0, &txt_ptr.1, 0);
    } 
}

pub fn print (str:&str) {

    // Unset ptr
    unsafe {setPtr(&t_ptr.0, &t_ptr.1, 1);}

    let color = 0xF;
    unsafe {
        drawString(t_ptr.0, t_ptr.1, str, color, 4)
    };

    let count = str.chars().count();
    match Option::Some(str.as_bytes().last()) {
        Some(x) if x.unwrap() == &("\n".as_bytes()[0]) => unsafe {
            t_ptr.1 = t_ptr.1 + FONT_HEIGHT;
            t_ptr.0 = 10;
            setPtr(&t_ptr.0, &t_ptr.1, 0);  
        },
        _ =>unsafe {
                t_ptr.0 = t_ptr.0 + ((FONT_WIDTH) * count as u32);
                setPtr(&t_ptr.0, &t_ptr.1, 0);  
            },
    }

}

pub fn println (str:&str) {

    if str.chars().last().unwrap() == "\n".as_bytes()[0] as char {
        print(str);
    } else {
        print(str);
        print("\n");
    }
}

fn setPtr (&x:&u32, &y:&u32, mode:u8) {

    let color = if mode == 0 {0xF} else {0x0}; // white

    for r in 0..8 {
        for c in 0..8 {
            drawPixel(&(x+c), &(y+r), color); 
        }
    }
}
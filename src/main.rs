#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
// #![feature(default_alloc_error_handler)]

use core::{panic::PanicInfo, fmt::write};
use core::arch::{global_asm, asm};
use core::fmt::Write;

// Include the bootloader when Linking
global_asm!(include_str!("boot.S"));


mod io;
mod mb;
mod fb;
mod terminal;
// mod serial;
// mod cmd;
// mod game;
mod multicore;
mod write_to;

use crate::multicore::*;
use crate::io::*;
use crate::fb::*;

#[no_mangle]
fn core0_main(){
    loop{}
    // start_core2(core1_main);
}

#[no_mangle]
fn core1_main(){
    clear_core1();
}

#[no_mangle]
fn core2_main(){
    clear_core2();
    // start_core2(core1_main);
}
#[no_mangle]
fn core3_main(){
    clear_core3();
    // start_core2(core1_main);
}

mod boot{
    use core::arch::global_asm;
    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern fn main(){
    

    // Start up COMs
    uart_init();
    fb_init();

    uart_writeText("Started\n");
    uart_writeHex(&0xFFFF_FFFF);


    // FrameBuffer Init
    drawString(10, 10, "Smash or Pass", 0x0F, 1);
    drawRect(100, 100, 200, 200, 0xAA, 0);
    drawChar(0x41, 150, 150, 0x0F, 2);

    // core0_main();
    loop{    }

    // start_core1(core1_main as *mut usize);
    // start_core2(core2_main as *mut usize);
    // start_core3(core3_main as *mut usize);
}    

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_writeText("PANIC!\n");
    loop {}
}
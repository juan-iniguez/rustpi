// #include "multicore.h"

use crate::io::*;

use core::{arch::asm, ptr::{self, write_volatile}};

extern "C" {
    static spin_cpu0:usize;
    static spin_cpu1:usize;
    static spin_cpu2:usize;
    static spin_cpu3:usize;
}

#[no_mangle]
pub fn store32(address:*mut usize, value:usize){
    unsafe{
        *address = value; 
    }
}

// void store32(unsigned long address, unsigned long value)
// {
//     *(unsigned long *) address = value;
// }

#[no_mangle]
pub fn load32(address:usize) -> usize {
    unsafe{
        return *(address as *mut usize);
    }
}

// unsigned long load32(unsigned long address)
// {
//     return *(unsigned long *) address;
// }

#[no_mangle]
pub fn start_core1(func:*mut usize){
    unsafe{
        store32(0xE0 as *mut usize, func as usize);
        asm!("sev");
    }
}

// C CODE
// Main stores *(func core1_main) into x0
// Calls start_core1
// start_core1 
// load address of *spin_cpu1 into x8;
// stores x0(*core1_main) into address of [x8](*spin_cpu1)
//  SEV

// RUST
// Main stores address *(func core1_main) into x0;
// calls start_core1
// Start_core1

// move x0 into x1
// store x1(*core1_main) into stack pointer + #8;

// load address of spin_cpu1 into x0
// branch to store32

// store32
// store x0(*spin_cpu1) into stack pointer
// store x1(*core1_main) into stack pointer + 8;
// load address of [x0](*spin_cpu1) into w8;
// store x1(*core1_main) into address of [x8](*spin_cpu1);
// return

// SET EVENT

#[no_mangle]
pub fn start_core2(func:*mut usize){
    unsafe{
        store32(0xe8 as *mut usize, func as usize);
        asm!("sev");
    }
}

#[no_mangle]
pub fn start_core3(func:*mut usize){
    unsafe{
        store32(0xF0 as *mut usize, func as usize);
        asm!("sev");
    }
}


// void start_core1(void (*func)(void))
// {
//     store32((unsigned long)&spin_cpu1, (unsigned long)func);
//     asm volatile ("sev");
// }

#[no_mangle]
pub fn clear_core1() {
        store32(0xE0 as *mut usize, 0)
}

#[no_mangle]
pub fn clear_core2() {
        store32(0xe8 as *mut usize, 0)
}

#[no_mangle]
pub fn clear_core3() {
        store32(0xf0 as *mut usize, 0)
}

// void clear_core1(void) 
// {
//     store32((unsigned long)&spin_cpu1, 0);
// }


// void start_core2(void (*func)(void))
// {
//     store32((unsigned long)&spin_cpu2, (unsigned long)func);
//     asm volatile ("sev");
// }

// void start_core3(void (*func)(void))
// {
//     store32((unsigned long)&spin_cpu3, (unsigned long)func);
//     asm volatile ("sev");
// }


// void clear_core2(void) 
// {
//     store32((unsigned long)&spin_cpu2, 0);
// }

// void clear_core3(void) 
// {
//     store32((unsigned long)&spin_cpu3, 0);
// }

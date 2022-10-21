use crate::io::*;

//// The buffer must be 16-byte aligned as only the upper 28 bits of the address can be passed via the mailbox
#[repr(C, align(16))]
pub struct mbox_struc(pub[u32;36]);
#[no_mangle]
pub static mut mbox:mbox_struc = mbox_struc([0;36]);
#[no_mangle]
pub static MBOX_REQUEST:u32 = 0x0000_0000;
pub static VIDEOCORE_MBOX_READ:u32 = PERIPHERAL_BASE + 0x0000_B880;
pub static MBOX_POLL:u32      = VIDEOCORE_MBOX_READ  + 0x10;
pub static MBOX_SENDER:u32    = VIDEOCORE_MBOX_READ  + 0x14;
pub static MBOX_STATUS:u32    = VIDEOCORE_MBOX_READ  + 0x18;
pub static MBOX_CONFIG:u32    = VIDEOCORE_MBOX_READ  + 0x1C;
pub static MBOX_WRITE:u32     = VIDEOCORE_MBOX_READ  + 0x20;
pub static MBOX_RESPONSE_FULL:u32  = 0x80000000;
pub static MBOX_EMPTY:u32     = 0x4000_0000;

// MBOX_CH_POWER = 0,
pub static MBOX_CH_FB:u32    = 1;
pub static MBOX_CH_VUART:u32 = 2;
pub static MBOX_CH_VCHIQ:u32 = 3;
pub static MBOX_CH_LEDS:u32  = 4;
pub static MBOX_CH_BTNS:u32  = 5;
pub static MBOX_CH_TOUCH:u32 = 6;
pub static MBOX_CH_COUNT:u32 = 7;
pub static MBOX_CH_PROP:u32  = 8; // Request from ARM for response by VideoCore

pub static MBOX_TAG_SETPOWER:u32   = 0x28001;
pub static MBOX_TAG_SETCLKRATE:u32 = 0x38002;

pub static MBOX_TAG_SETPHYWH:u32   = 0x48003;
pub static MBOX_TAG_SETVIRTWH:u32  = 0x48004;
pub static MBOX_TAG_SETVIRTOFF:u32 = 0x48009;
pub static MBOX_TAG_SETDEPTH:u32   = 0x48005;
pub static MBOX_TAG_SETPXLORDR:u32 = 0x48006;
pub static MBOX_TAG_GETFB:u32      = 0x40001;
pub static MBOX_TAG_GETPITCH:u32   = 0x40008;
pub static MBOX_TAG_GETSERIAL:u32  = 0x10004;

pub static MBOX_TAG_LAST :u32      = 0;

pub fn mbox_read(ch:u8) -> u32 {
    loop {
        // IS THERE A REPLY?
        while mmio_read(MBOX_STATUS as u32) & MBOX_EMPTY as u32 != 0 {};

        let mut data = mmio_read(VIDEOCORE_MBOX_READ as u32);
        let data_ch = (data & 0xF) as u8;
        data >>= 4;
        if data_ch == ch {
            return data;
        }
    }
}

pub fn mbox_write(ch:u8) {
    unsafe{
        let r = &mut mbox.0 as *mut u32 as u32 & !0xF | (ch & 0xF) as u32;
        
        // Wait until we can write
        while mmio_read(MBOX_STATUS as u32) & MBOX_RESPONSE_FULL as u32 != 0 {}
        
        // Write the address of our buffer to the mailbox with the channel appended
        mmio_write(MBOX_WRITE as u32, r as u32);

    }
}
    
pub fn mbox_call(ch:u8) -> u32 {

    // 28-bit address (MSB) and 4-bit value (LSB)
    let r:u32 = unsafe{ (&mut mbox.0 as *mut u32) as u32 & !0x0F | (ch & 0x0F) as u32};

    // Wait until we can write
    while mmio_read(MBOX_STATUS as u32) & MBOX_RESPONSE_FULL as u32 != 0 {};

    // Write the address of our buffer to the mailbox with the channel appended
    mmio_write(MBOX_WRITE as u32, r as u32);
    loop {
        // Is there a reply?
        while mmio_read(MBOX_STATUS as u32) & MBOX_EMPTY as u32 != 0 {};
        // Is it a reply to our message?
        if mmio_read(VIDEOCORE_MBOX_READ as u32) == r {
            if unsafe {mbox.0[1] == MBOX_RESPONSE_FULL as u32} {
                return 1;
            }else{
                return 0;
            }
        }
    }
}
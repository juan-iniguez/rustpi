use core::arch::asm;

use crate::io::*;
use crate::mb::*;
use crate::terminal::*;

#[no_mangle]
pub static mut width:u32 = 0;
#[no_mangle]
pub static mut height:u32 = 0;
#[no_mangle]
pub static mut pitch:u32 = 0;
#[no_mangle]
static mut isrgb:u32 = 0;
#[no_mangle]
pub static mut fb:*mut u32 = 0 as *mut u32;

#[no_mangle]
pub fn fb_init() {
    unsafe{
        mbox.0[0] = 35*4; // Length of message in bytes
        mbox.0[1] = MBOX_REQUEST;

        // First TAG
        mbox.0[2] = MBOX_TAG_SETPHYWH as u32; // Tag identifier
        mbox.0[3] = 8; // Value size in bytes
        mbox.0[4] = 0; // 0b(0)0000000_0000000 ;; b31 clear = REQ, b31 set = RES;
        mbox.0[5] = 1920; // Value(width)
        mbox.0[6] = 1080; // Value(height)
    
        mbox.0[7] = MBOX_TAG_SETVIRTWH as u32;
        mbox.0[8] = 8;
        mbox.0[9] = 8;
        mbox.0[10] = 1920;
        mbox.0[11] = 1080;
    
        mbox.0[12] = MBOX_TAG_SETVIRTOFF as u32;
        mbox.0[13] = 8;
        mbox.0[14] = 8;
        mbox.0[15] = 0; // Value(x)
        mbox.0[16] = 0; // Value(y)
    
        mbox.0[17] = MBOX_TAG_SETDEPTH as u32;
        mbox.0[18] = 4;
        mbox.0[19] = 4;
        mbox.0[20] = 32; // Bits per pixel

        mbox.0[21] = MBOX_TAG_SETPXLORDR as u32;
        mbox.0[22] = 4;
        mbox.0[23] = 4;
        mbox.0[24] = 1; // RGB
    
        mbox.0[25] = MBOX_TAG_GETFB as u32;
        mbox.0[26] = 8;     // Value buffer size (bytes)
        mbox.0[27] = 8;     // Req. + value length (bytes)
        mbox.0[28] = 4096;    // Frame Buffer Base
        mbox.0[29] = 0;     // Screen Size (Pixels in Screen)
    
        mbox.0[30] = MBOX_TAG_GETPITCH as u32;
        mbox.0[31] = 4;
        mbox.0[32] = 4;
        mbox.0[33] = 0; // Bytes per line
    
        mbox.0[34] = MBOX_TAG_LAST as u32;

        //     // Check call is successful and we have a pointer with depth 32

        if ((mbox_call(MBOX_CH_PROP as u8)!= 0) && (mbox.0[20] == 32)  && (mbox.0[28] != 0)) != false  {
            mbox.0[28]  &= 0x3FFF_FFFF;     // Convert GPU address to ARM address
            width       = mbox.0[10];       // Actual physical width
            height      = mbox.0[11];       // Actual physical height
            pitch       = mbox.0[33];       // Number of bytes per line
            isrgb       = mbox.0[24];       // Pixel order
            fb          = mbox.0[28] as *mut u32;
        } else {
            uart_writeText("F#1\n");
        }
    }
}

#[no_mangle]
pub fn drawPixel(x:u32, y:u32, attr:u8) {
    unsafe{
        let offs:u32 = (y * pitch as u32) + (x * 4);
        // FrameBuffer pointer + offset Pointer -> Pointer to 32bit pixel = Color;
        *((fb as u32 + offs as u32) as *mut u32) = vgapal[(attr & 0x0f) as usize];
    }
}

#[no_mangle]
pub fn drawRect(x1:u32, y1:u32, x2:u32, y2:u32, attr:u8, fill:u32){

    let mut y = y1;

    while y <= y2 {
        let mut x = x1;
        while x <= x2 {
            if(x == x1) || (x == x2) || (y == y1) || (y == y2) {
                drawPixel(x, y, attr);
            }else if fill != 0 {
                drawPixel(x, y, attr & 0xf0 >> 4);
            }
            x+=1;
        }
        y+=1;
    }
}

#[no_mangle]
pub fn drawLine(x1:i32, y1:i32, x2:i32, y2:i32, attr:u8) {

    let (dx, dy, mut p, mut x, mut y):(i32,i32,i32,i32,i32);
    dx = x2-x1;
    dy = y2-y1;
    x = x1;
    y = y1;
    p = 2*dy-dx;
    while x<x2 {
        if p >= 0 {
            drawPixel((x as u32), (y as u32), attr);
            y+=1;
            p = p+2*dy-2*dx;
        } else {
            drawPixel((x as u32), (y as u32), attr);
            p = p+2*dy;
        }
        x+=1
    }
}

#[no_mangle]
pub fn drawCircle(x0:i32, y0:i32, radius:i32, attr:u8, fill:i32){
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;
    while x>=y {
        if fill != 0{
            drawLine(x0 - y, y0 + x, x0 + y, y0 + x, (attr & 0xf0) >> 4);
            drawLine(x0 - x, y0 + y, x0 + x, y0 + y, (attr & 0xf0) >> 4);
            drawLine(x0 - x, y0 - y, x0 + x, y0 - y, (attr & 0xf0) >> 4);
            drawLine(x0 - y, y0 - x, x0 + y, y0 - x, (attr & 0xf0) >> 4);
        }
        drawPixel(((x0 - y) as u32), ((y0 + x) as u32), attr);
        drawPixel(((x0 + y) as u32), ((y0 + x) as u32), attr);
        drawPixel(((x0 - x) as u32), ((y0 + y) as u32), attr);
        drawPixel(((x0 + x) as u32), ((y0 + y) as u32), attr);
        drawPixel(((x0 - x) as u32), ((y0 - y) as u32), attr);
        drawPixel(((x0 + x) as u32), ((y0 - y) as u32), attr);
        drawPixel(((x0 - y) as u32), ((y0 - x) as u32), attr);
        drawPixel(((x0 + y) as u32), ((y0 - x) as u32), attr);
        if err <=0 {
            y += 1;
            err += 2*y +1;
        }
        if err > 0 {
            x -= 1;
            err -= 2*x + 1;
        }
    }
}

#[no_mangle]
pub fn drawChar(ch:u8, x:u32, y:u32, attr:u8, zoom:u32) {    
    // There's 8 bytes per glyph (64 bits)
    // Make a pointer to the array of glyphs and point it using the char 
    // It checks if the char is out of range FONT NUMGLYPHS

    // let glyph = font[ch as usize];
    // unsigned char *glyph = (unsigned char *)&font + (ch < FONT_NUMGLYPHS ? ch : 0) * FONT_BPG;
    
    
    // let mut glyph:*mut u8 = (
    //     &mut font as *mut [u8;8] as *mut u8 as u32 + 
    //     (if ch < FONT_NUMGLYPHS {
    //         ch as u32
    //     } else {0}) * FONT_BPG as u32) as *mut u8;

    let mut glyph:*mut u8 = ((font[0][0]) as *mut u8 as u32 + (if ch < FONT_NUMGLYPHS {ch as u32} else {0}) * FONT_BPG as u32) as *mut u8;

    for i in 1..FONT_HEIGHT*zoom {
        for j in 0..FONT_WIDTH*zoom{
            let mask:u8 = 1 << j/zoom;
            unsafe{
                let col:u8 = if *glyph & mask != 0 {attr & 0x0F} else {(attr & 0xF0) >> 4 };
                drawPixel(x+j, y+i, col);
            }
        }
        // glyph += (i%zoom) ? 0 : FONT_BPL;
        glyph = (glyph as u32 + if i%zoom == 0 {0} else {FONT_BPL}) as *mut u8;
    }




}

#[no_mangle]
pub fn drawString(x:u32, y:u32, string:&str, attr:u8, zoom:u32) {
    
    let mut string_iterator = string.chars().fuse();
    let mut x1 = x;
    let mut y1 = y;

    while let Some(T) = string_iterator.next() {
        if T == "\n".as_bytes()[0] as char {
            y1 += (FONT_HEIGHT*FONT_SIZE)*zoom;
            x1 = x;
            drawChar(T as u8, x1, y1, attr, zoom);
        } else {
            drawChar(T as u8, x1, y1, attr, zoom);
            x1 += (FONT_WIDTH*FONT_SIZE)*zoom;
        }
    }


    // for _ in 0..str_length{
    //     unsafe{
    //         if *str_ptr as char == '\r' {
    //             x1 = 0;
    //         } else if *str_ptr as char == '\n' {
    //             x1=0; y1 += FONT_HEIGHT*zoom;
    //         } else {
    //             drawChar(*str_ptr, &x, &y, attr, zoom);
    //             x1 += FONT_HEIGHT*zoom;
    //         }
    //         str_ptr = (str_ptr as u32 + 1) as *const u8;
    //     }
    // }

    // while (*s) {
    //     if (*s == '\r') {
    //        x = 0;
    //     } else if(*s == '\n') {
    //        x = 0; y += (FONT_HEIGHT*zoom);
    //     } else {
    //    drawChar(*s, x, y, attr, zoom);
    //        x += (FONT_WIDTH*zoom);
    //     }
    //     s++;
    //  }
 

}

#[no_mangle]
pub fn wait_msec(n:u32){
    let ( mut f,mut t,mut r):(u32,u32,u32);
    unsafe{
        // Get the current counter frequency 54,000,000
        asm!("mrs {:x}, cntfrq_el0", out(reg) f);
        // Read the current counter (eg: 0)
        asm!("mrs {:x}, cntpct_el0", out(reg) t);
        // Calculate expire value for counter
        t+=((f/1000)*n)/1000;
        loop{
            // Get the counter and store it in r
            asm!("mrs {:x}, cntpct_el0", out(reg) r);
            // When r is > than t break
            if r>t {
                break;
            }
        }
    }
}

// void wait_msec(unsigned int n)
// {
//     register unsigned long f, t, r;
//     // Get the current counter frequency
//     asm volatile ("mrs %0, cntfrq_el0" : "=r"(f));
//     // Read the current counter
//     asm volatile ("mrs %0, cntpct_el0" : "=r"(t));
//     // Calculate expire value for counter
//     t+=((f/1000)*n)/1000;
//     do{asm volatile ("mrs %0, cntpct_el0" : "=r"(r));}while(r<t);
// }

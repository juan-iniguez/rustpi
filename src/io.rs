use core::ptr;

// Base Address for all Peripherals
pub  const PERIPHERAL_BASE:u32 = 0xFE00_0000;
// GPIO Base Address
pub const GPIO_BASE:u32 = PERIPHERAL_BASE + 0x20_0000;

pub const GPFSEL0:u32         = PERIPHERAL_BASE + 0x200000;
pub const GPSET0:u32          = PERIPHERAL_BASE + 0x20001C;
pub const GPCLR0:u32          = PERIPHERAL_BASE + 0x200028;
pub const GPPUPPDN0:u32       = PERIPHERAL_BASE + 0x2000E4;
pub const MAX_PIN:u32         = 53;
pub const FUNCTION_ALT5:u32   = 2;
pub const Pull_None:u32       = 0;
pub const BASE :u32       = PERIPHERAL_BASE + 0x215000;
pub const ENABLES  :u32   = BASE + 4;
pub const MU_IO_REG :u32  = BASE + 64;
pub const MU_IER_REG :u32 = BASE + 68;
pub const MU_IIR_REG:u32  = BASE + 72;
pub const MU_LCR_REG:u32  = BASE + 76;
pub const MU_MCR_REG :u32 = BASE + 80;
pub const MU_LSR_REG:u32  = BASE + 84;
pub const MU_CNTL_REG:u32 = BASE + 96;
pub const MU_STAT_REG:u32 = BASE + 100;
pub const MU_BAUD_REG:u32 = BASE + 104;
pub const UART_CLOCK:u32  = 500_000_000; // We set the RaspberryPi Config.txt to 500 minimum core freq.
pub const UART_MAX_QUEUE:u32  = 16 * 1024;




fn gpio_call(pin_number:u32, value:u32, base:u32, field_size:u32, field_max:u32) -> u32 {

    // Create a mask to tell what Register we are using
    // Max Pin is 53, so that is our field_size
    //  1 << 0x03
    let field_mask:u32 = (1 << field_size) - 1;

    if pin_number > field_max {
        return 0;
    }
    if value > field_mask {
        return 0;
    }

    let num_fields:u32 = 32 / field_size;
    let reg:u32 = base + ((pin_number / num_fields) * 4);
    let shift:u32 = (pin_number % num_fields) * field_size;

    let mut curval:u32 = mmio_read(reg) as u32;
    curval &= !(field_mask << shift);
    curval |= value << shift;
    mmio_write(reg, curval);

    return 1;

}

fn gpio_set ( pin_number:u32, value:u32) -> u32 { return gpio_call(pin_number, value, GPSET0 as u32, 1, MAX_PIN as u32); }

fn gpio_clear ( pin_number:u32, value:u32) -> u32 { return gpio_call(pin_number, value, GPCLR0 as u32, 1, MAX_PIN as u32); }

fn gpio_pull ( pin_number:u32, value:u32) -> u32 { return gpio_call(pin_number, value, GPPUPPDN0 as u32, 2, MAX_PIN as u32); }

fn gpio_function ( pin_number:u32, value:u32) -> u32 { return gpio_call(pin_number, value, GPFSEL0 as u32, 3, MAX_PIN as u32) }

fn gpio_useAsAlt5 (pin_number:u32) {
    gpio_pull(pin_number, Pull_None as u32);
    gpio_function(pin_number, FUNCTION_ALT5 as u32);
}

pub fn AUX_MU_BAUD (baud:u32) -> u32 {return (UART_CLOCK as u32/(baud*8))-1}

pub fn mmio_read(reg:u32) -> u32 {
    unsafe{
        return ptr::read_volatile(reg as *mut u32);
    }
}

pub fn mmio_write(reg:u32, val:u32) {
    unsafe{
        return ptr::write_volatile(reg as *mut u32, val as u32)
    }
}

pub fn uart_init() {
    
    // MMIO WRITE to Register UART ENABLES STARTS THE COMMUNICATION WHEN BIT 0 = 1
    mmio_write(ENABLES as u32, 1);
    // WRITE TO REGISTER INTERRUPT ENABLE, BIT 0 ENABLES INTERRUPT FOR TX and RX
    mmio_write(MU_IER_REG as u32, 0);
    // WRITE TO REGISTER TO CONTROL THE UART FLOW, DONT RECEIVE OR SEND ANYTHING IN PROGRESS
    mmio_write(MU_CNTL_REG as u32, 0);
    // SET UART IN 8 BIT MODE
    mmio_write(MU_LCR_REG as u32, 3);
    // UART_RTS line is HIGH = 0
    mmio_write(MU_MCR_REG as u32, 0);
    // Again set Interrupt Enable to Enabled?
    mmio_write(MU_IER_REG as u32, 0);
    // <Not possible> Interrupts
    mmio_write(MU_IIR_REG as u32, 0xC6);
    // Set Baud Rate using a Function AUX MU BAUD
    mmio_write(MU_BAUD_REG as u32, AUX_MU_BAUD(115200));
    // Set the UART Pins to TX and RX respectively
    // ALT5 is the function of these pins for TX and RX
    gpio_useAsAlt5(14);
    gpio_useAsAlt5(15);

    // WRITE TO REGISTER TO CONTROL THE UART FLOW, ALLOW TO RECEIVE OR SEND ANYTHING IN PROGRESS
    mmio_write(MU_CNTL_REG as u32,3);

}

// unsigned int uart_isReadByteReady()  { return mmio_read(AUX_MU_LSR_REG) & 0x01; }

fn uart_isReadByteReady() -> u32 { return (mmio_read(MU_LSR_REG as u32)) & 0x01}
fn uart_isWriteByteReady() -> u32 {
    // Read the Data Status. If '0010_0000' is read, then FIFO can accept at least one byte
    return mmio_read(MU_LSR_REG as u32) & 0x20;
}

fn uart_writeByteBlockingActual(ch:char) {
    // Check if the UART is ready to send
    while uart_isWriteByteReady() == 0 {};
    // Send if good
    mmio_write(MU_IO_REG as u32, ch as u32);
}

pub fn uart_read() -> u32 {
    let mut x:u8 = 0;
    while uart_isReadByteReady() == 0 { 
        if x == 5 {
            // TimedOut
            return 0;
        } x+=1;
    };
    return mmio_read(MU_IO_REG as u32);
}

pub fn uart_writeText(buffer:&str) {

    // Grab buffer String and make it an iterable type. 
    // This will allow to iterate over the string
    // Without having to use the address of the string character.
    let mut x  = buffer.chars().fuse();

    // Some type C, will be a char type. When *c = &x gets read, it will go to the next character   
    while let Some(c) = x.next() {
        if c == '\n' {
            uart_writeByteBlockingActual('\n');
        } else{
            uart_writeByteBlockingActual(c);
        }
     }
}

pub fn uart_writeHex(buffer:&u32) {

    let x = *buffer;

    for i in 0..4 {
        let shifter = ((x >> (i*2)*4) & 0xFF) as u8;
        while uart_isWriteByteReady() == 0 {};
        mmio_write(MU_IO_REG as u32, shifter as u32);
    }
}
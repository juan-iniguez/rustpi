
use crate::mb::*;
use crate::io::*;

#[repr(C, align(16))]
pub struct mbox_s_struc(pub[u32;8]);
pub static mut mbox_s:mbox_s_struc = mbox_s_struc([0;8]);

#[no_mangle]
pub fn get_serial_init() -> [u32;2] {
    unsafe{
        mbox_s.0[0] = 8*4; // BUFFER SIZE
        mbox_s.0[1] = MBOX_REQUEST; //REQUEST

        mbox_s.0[2] = MBOX::MBOX_TAG_GETSERIAL as u32; // WHAT DO YOU WANNA REQUEST?

        mbox_s.0[3] = 8;  // VALUE OF BUFFER SIZE IN BYTES
        mbox_s.0[4] = 8;  // REQUEST CODES / RESPONSE CODES 8 = (REQUEST | 8 BYTES VALUE RESPONSE )
        
        mbox_s.0[5] = 0;  // VALUE BUFFER (4 BYTES)
        mbox_s.0[6] = 0;  // VALUE BUFFER (4 BYTES)

        mbox_s.0[7] = MBOX::MBOX_TAG_LAST as u32; // END TAG
    }

    mbox_write(MBOX::MBOX_CH_PROP as u8);
    
    let serial_num:[u32;2] = unsafe{[mbox_s.0[6], mbox_s.0[5]]};

    return serial_num;

}
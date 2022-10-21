
use crate::fb::{drawRect, drawCircle, drawPixel};

pub static mut paddle_pos:[u32;2]= [500, 900];
pub static paddle_size:[u32;2] = [200,20];
pub static mut ball_pos:[i32;2] = [900,600];
static mut ball_size:i32 = 5;
pub static paddle_id:u8 = 0x01;
pub static ball_id:u8 = 0x02;
static mut ball_dir:u8 = 0x02;

pub fn game_init() {
    unsafe {

        drawRect(paddle_pos[0], paddle_pos[1], paddle_pos[0] + paddle_size[0], paddle_pos[1] + paddle_size[1], 0xff, 1);

        drawCircle(ball_pos[0], ball_pos[1], ball_size, 0xFF, 1);
    }
}

pub fn moveObject( xoffs:i32, yoffs:i32, objectID:u8 ) {
    let _object = Some(objectID);
    let speed = 5;

    match _object {
        // Paddle To the left
        Some(x) if (x == paddle_id) && (xoffs < 0)  => 

        if paddle_collision() != (true, false) {
            unsafe {
                let front = ((paddle_size[0]+paddle_pos[0]) as i32 + xoffs) as u32;
                let back = (paddle_pos[0] as i32 + xoffs) as u32;
                // Erase the Front
                drawRect(front, paddle_pos[1], paddle_size[0] + paddle_pos[0], paddle_pos[1] + paddle_size[1], 0x0, 1);
                // Fill the back
                drawRect(back, paddle_pos[1], paddle_pos[0], paddle_pos[1] + paddle_size[1], 0xF, 1);

                paddle_pos = [(paddle_pos[0] as i32 + xoffs) as u32, paddle_pos[1]];
            }
        },
        // Paddle To the Right
        Some(x) if (x == paddle_id) && (xoffs > 0) => 
        if paddle_collision() != (false, true) {
            unsafe{

                let front = ((paddle_size[0]+paddle_pos[0]) as i32) as u32;
                let back = paddle_pos[0];


                // Erase the Back
                drawRect(back, paddle_pos[1], back+xoffs as u32, paddle_pos[1] + paddle_size[1] , 0x0, 1);
                // Fill the Front
                drawRect(front, paddle_pos[1], front+xoffs as u32, paddle_pos[1] + paddle_size[1], 0xF, 1);

                // Update the Pos
                paddle_pos = [paddle_pos[0] + xoffs as u32, paddle_pos[1]];

            }
        }    ,
        // Ball
        Some(x) if (x == ball_id) =>
        unsafe {
            
            // Clear the circle
            drawCircle(ball_pos[0], ball_pos[1], ball_size, 0x00, 1);

            // Collisions
            let xball = ball_collision(ball_dir, speed);

            // Draw the circle
            drawCircle(ball_pos[0] + (xoffs*xball.0 as i32), ball_pos[1] + (yoffs*xball.1 as i32), ball_size, 0xFF, 1);

            ball_pos[0] += xoffs*xball.0 as i32;
            ball_pos[1] += yoffs*xball.1 as i32;
        },
        _ => ()
    }

    // unsafe {drawPixel(&(ball_pos[0] as u32), &(ball_pos[1] as u32), 0x4)};

}

fn paddle_collision() -> (bool, bool) {
    unsafe{
        if paddle_pos[0] <= 0{
            return (true,false)
        }else if paddle_pos[0] + paddle_size[0] >= 1920{
            return (false,true)
        }else{
            (false,false)
        }
    }
}

fn ball_collision (bd:u8, speed:i32) -> (i8, i8) {

        let L =  unsafe{ball_pos[0]-ball_size};
        let R =  unsafe{ball_pos[0]+ball_size};
        let U =  unsafe{ball_pos[1]-ball_size};
        let D =  unsafe{ball_pos[1]+ball_size};

    // Check for collisions with screen or paddle
    let ball_dir_sel = Some(bd);
    
    match ball_dir_sel {
        // South East
        Some(x) if (x == 0x00) =>

        if unsafe{ball_pos[1] >= 1000} {
            reset_game();
            return (0,0);
        }else


        // Check for collision with Paddle or right side of Screen
        if unsafe{(D as u32 >= paddle_pos[1]-speed as u32) && ((L as u32 >= paddle_pos[0] as u32 )&& (R as u32 <= paddle_pos[0]+paddle_size[0]))} {
            unsafe {
                ball_dir = 0x03;
            }
            return (1,-1)
        } else if R >= 1915 {
            unsafe{ ball_dir = 0x01}
            return (-1,1)
        }else {
            return (1,1)
        }
        ,
        // South West
        Some(x) if (x == 0x01) =>

        if unsafe{ball_pos[1] >= 1000} {
            reset_game();
            return (0,0);
        }else
        if unsafe{(D as u32 >= paddle_pos[1]-speed as u32) && ((L as u32 >= paddle_pos[0] as u32 )&& (R as u32 <= paddle_pos[0]+paddle_size[0]))} {
            unsafe{
                ball_dir = 0x02;
            }
            return (-1,-1)
        }else if L <= 5{
            unsafe{
                ball_dir = 0x00;
            }
            return (1,1)
        }else {
            return (-1,1)
        }
        ,
        // North West
        Some(x) if (x == 0x02) =>

        if U as u32 <= 5{
            unsafe{ball_dir = 0x01;}
            return (-1,1)
        }else if L <= 5 {
            unsafe{ball_dir = 0x03};
            return (1,-1)
        }else {
            return (-1,-1)
        }
        ,
        // North East
        Some(x) if (x == 0x03) =>

        if U as u32 <= 5 {
            unsafe{ball_dir = 0x00;}
            return (1,1)
        } else if R >= 1915 {
            unsafe{ball_dir = 0x02}
            return (-1,-1)
        } else {
            return (1,-1)
        }
        ,
        _ => (0, 0),
    }
}

pub fn reset_game(){
    unsafe{
        ball_pos = [1920/2, 1080/2];
        ball_dir = 0x02;
        paddle_pos[0] = 1920/2;
        
    }
}
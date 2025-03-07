use std::time::{Instant, Duration};

fn main() {
    let mut lunar_pos_y: f64 = 100.;

    let mut velocity_y : f64 = 1.;
    let gravity : f64 = -9.81;

    let mut timer = Instant::now();
    let mili_second = Duration::from_millis( 1 );
    let second = Duration::from_secs( 1 );
    let mut display_timer = Instant::now();

    loop {
        if (timer.elapsed() > mili_second) && ( lunar_pos_y > 0.0 ) {
            lunar_pos_y = lunar_pos_y + velocity_y / 1000.0;
            velocity_y += gravity / 1000.0;
            timer = Instant::now();
        }
        
        if lunar_pos_y <= 0.0{
            break;
        }
        
        if display_timer.elapsed() > second{
            display_lander( lunar_pos_y );
            display_timer = Instant::now();
        }
    }
}

fn display_lander( _position : f64 ){
    let lander_row: i64 = (_position / 10.0).floor() as i64;
    
    for i in 0..10 {
        if i == 9 - lander_row{
            if i == 9 {
                println!( "-----x-----");
            } else {
                println!( "     x     ");
            }
        } else if i == 9{
            println!( "-----------" );
        } else {
            println!( "           " );
        }
    }
}
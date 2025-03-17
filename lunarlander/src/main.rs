#[derive(Debug)]
struct Position{
    x_coordinate: f64,
    y_coordinate: f64,
}
#[derive(Debug)]
struct Lander{
    pos: Position,
    x_velocity: f64,
    y_velocity: f64,
}

fn main(){
    println!( "Lunar Lander" );
    println!( "Playing with Structs" );

    let position = Position { x_coordinate: 10., y_coordinate: 10. };
    let lander = Lander{ pos: position, x_velocity: 10., y_velocity: 10. };

    change_speed(&lander);
    change_position(&lander.pos);

    println!( "Lander: {lander:#?}");
}

fn change_speed( lander : &Lander ) -> f64{
    lander.x_velocity + lander.y_velocity
}

fn change_position( position : &Position ) -> f64{
    position.x_coordinate + position.y_coordinate
}
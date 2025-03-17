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

impl Lander {
    fn slowdown( &mut self ){
        self.x_velocity = self.x_velocity - 1.;
        self.y_velocity = self.y_velocity - 1.;
    }
}

impl Lander {
    fn upordown( &self ) -> bool{
        self.y_velocity > 0.
    }
}

impl Lander {
    fn standing() -> Self{
        Self{
            pos: Position{ x_coordinate: 0., y_coordinate: 0. },
            x_velocity: 0.,
            y_velocity: 0.,
        }
    }
}

fn main(){
    println!( "Lunar Lander" );
    println!( "Playing with Structs" );

    let mut lander = Lander::standing();

    change_speed(&lander);
    change_position(&lander.pos);

    println!( "Lander: {lander:#?}");

    lander.slowdown();
    lander.upordown();

    println!( "Lander: {lander:#?}" );
}

fn change_speed( lander : &Lander ) -> f64{
    lander.x_velocity + lander.y_velocity
}

fn change_position( position : &Position ) -> f64{
    position.x_coordinate + position.y_coordinate
}
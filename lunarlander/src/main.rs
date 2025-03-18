#[derive(Debug)]
struct Table{
    width : f64,
    height : f64,
    deepth : f64,
}

impl Table{
    fn volume( &mut self ) -> f64 {
        self.width = self.width + 1.;
        self.width * self.height * self.deepth
    }

    fn table( width : f64, height : f64 ) -> Table{
        Table{ 
            width : width,
            height : height,
            deepth : 100.,
        }
    }
}

fn main(){
    let t = Table::table( 100., 200. );
    let v = t.volume();

    println!( "{t:#?}" );
    println!( "{v}" ); 
}
fn main(){
    let mut s = String::from( "Hello" );
    
    let s_pointer = &mut s;

    modify_s( s_pointer );

    println!( "{}", s );
}

fn modify_s( test : &mut String ){
    test.push_str( " world" );
}
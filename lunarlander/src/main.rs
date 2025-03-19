enum Furnitures {
    Table{ width : f64, height : f64 },
}



fn main(){
    let b = Furnitures::Table {
        width : 100.,
        height : 100.
    };

}


fn modifyFurnitures( f : Furnitures ){
    f.Table.width
}
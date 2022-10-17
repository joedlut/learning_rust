struct Point<T>{
    x:T,
    y:T,
}


impl Point<f32>{
    fn distance_from_origin(&self)->f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}


fn main(){
    let p = Point{x:3.0,y:4.0};
    let distance = p.distance_from_origin();
    println!("distance is {}",distance);
}

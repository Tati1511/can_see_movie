
fn can_see_movie(age: i32, permission: bool) -> bool {
    (age >=17 && permission == false)||
    (age >=13 && permission != false)
    
}


fn main() {
    can_see_movie(17, false);
    println!("Go!");
}

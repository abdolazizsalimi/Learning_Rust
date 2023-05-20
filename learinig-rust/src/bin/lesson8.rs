


fn main(){
 let coord = (2,4);
 println!("{:?} , {:?}" , coord.0 , coord.1);
 let (x,y) = (5 , 7);
 println!("x: {:?}" , x);
 println!("y: {:?}" , y);
 let (name , age ) = ("Mazyar" , 18);
 println!("name: {:?}" , name);
 println!("age : {:?}" , age );

let favorites = ("red" , 16 , "TX","pizza","TV show" , "home");
let state = favorites.2;
let place = favorites.5;
let food = favorites.3;

println!("{:?}",state);
println!("{:?}",place);
println!("{:?}",food);

}
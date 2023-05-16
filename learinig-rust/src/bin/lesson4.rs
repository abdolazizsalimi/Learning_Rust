

fn main(){


    //  example of match 
    let some_bool = true;
    match some_bool {

        true => println!("it's ok"),
        false => println!("it's not ok")
    }


    // more example 
    let some_int = 4;
    match some_int {
        1=> println!("1"),
        2=> println!("2"),
        3=> println!("3"),
        4=> println!("4"),
        _=> println!("not valid number !!!!")
        
    }
}
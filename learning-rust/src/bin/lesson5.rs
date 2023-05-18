fn main()
{
let mut i = 3 ;
// loop in rust :
    loop {
        println!("{:?}" , i );
        if i == 0 {
            break;
        }
        i = i-1;
    }
    println!("loop done !!"); 
}
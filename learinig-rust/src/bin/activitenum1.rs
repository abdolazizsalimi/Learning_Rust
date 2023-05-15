fn sum(a: i32 ,b: i32) -> i32{
    a+b
}


fn display_result(res:i32){
    println!("{:?}" , res);

}

fn main(){
    let sum = sum(3,5);
    display_result(sum);
}
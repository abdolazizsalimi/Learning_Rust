


struct ShippingBox{
    width:i32,
    height:i32,
    depth:i32,
}
fn main(){
    let my_box = ShippingBox{
        width: 5,
        height: 3,
        depth: 2,
    };
    println!("my box size is depth:{:?}, height:{:?}, width:{:?}",my_box.depth , my_box.height , my_box.width);
}



enum Direction{
    Up,
    Down,
    Right,
    Left
}


fn which_way(go:Direction){
    match go{
        Direction::Up =>println!("go Up"),
        Direction::Down =>println!("go Down"),
        Direction::Right =>println!("go Right"),
        Direction::Left =>println!("go Left")
    }
}

fn main(){
    let go_up = Direction::Up;
    let go_down = Direction::Down;
    let go_right = Direction::Right;
    let go_left = Direction::Left;
    which_way(go_up);
    which_way(go_down);
    which_way(go_left);
    which_way(go_right);
}
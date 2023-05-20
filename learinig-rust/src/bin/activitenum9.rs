
fn print_massage(gt_100 :bool){
    match gt_100 {
        true => println!("the value is grader than 100"),
        _=> println!("the value is smallest than 100 "),
    }


}

fn main(){
let value = 100;
let is_gt_value = value > 100;
print_massage( is_gt_value);
}
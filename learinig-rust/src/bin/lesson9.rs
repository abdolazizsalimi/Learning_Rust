
enum Access{
    Admin,
    Manager,
    User, 
    Guest
}
fn main(){
    let access_level = Access::Admin;
//  more example : 
    // let access_level2 = Access::Manager;
    // let access_level3 = Access::User;
    // let access_level4 = Access::Guest;

let can_access_file = match access_level {
    Access::Admin => true,
    _ => false,
};
println!("can access: {:?}" , can_access_file);
}
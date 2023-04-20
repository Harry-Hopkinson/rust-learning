mod structs;
use structs::structs;

mod rectangle;
use rectangle::rectangle;

mod users;
use users::users;

fn main() {
    println!("------------------Structs------------------");
    structs();
    println!("------------------Rectangle------------------");
    rectangle();
    println!("------------------Users------------------");
    users();
}

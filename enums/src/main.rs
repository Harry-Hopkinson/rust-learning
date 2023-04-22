mod ip;
mod message;
mod money;

fn main() {
    println!("-------------------IP-------------------");
    ip::ip();
    println!("-------------------Message-------------------");
    message::message();
    println!("-------------------Money-------------------");
    money::money();
}

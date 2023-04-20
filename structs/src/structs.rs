#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn structs() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rect is {:?}", rec);
}

pub fn strings() {
    let data: &str = "intial data";
    let s: String = data.to_string();
    println!("{}", s);
    println!("{}", data);
    let d: String = String::from(data);
    println!("{}", d)
}

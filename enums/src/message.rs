pub fn message() {
    enum Message {
        Write(String),
    }

    impl Message {
        fn call(&self) {
            println!("Message called");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

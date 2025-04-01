// struct Message{
//     text: String
// }


// impl Message {
//     fn to_screen(&self) {
//         println!("{}", self.text);
//     }
// }


// fn main() {
//     let my_message = Message{ text: String::from("Hello world") };
//     my_message.to_screen();
// }




enum Message {
    Write(String)
}


impl Message {
    fn to_screen(&self) {
        match &self {
            Message::Write(m) => println!("{}", m),
        }
    }
}


fn main () {
    let my_message = Message::Write(String::from("Hello world"));
    my_message.to_screen();
}
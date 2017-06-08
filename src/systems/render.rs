use utility::Message as Message;
use utility::MessageBus as MessageBus;

pub struct Render{
    pub post_message: fn (&mut MessageBus, Message)
}

impl Render{
     pub fn handle_message(msg: &Message) {
        match msg {
            &Message::Start => println!("{:?}", *msg),
            &Message::Quit => println!("{:?}", msg),
            &Message::Pause => println!("{:?}", msg),
            _ => ()
        }
    }
}

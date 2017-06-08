use systems::Render;
use systems::Gui;
use systems::Input;
use utility::Message;

pub struct MessageBus{
    pub queue: Vec<Message>,
    pub render_message: fn(&Message)
}

impl MessageBus {
    pub fn post_message(&mut self, msg: Message){
        self.queue.push(msg)
    }

    pub fn update(&self){
        for ref msg in self.queue.iter(){
            (self.render_message)(msg);
        }
    }
}

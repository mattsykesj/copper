mod utility;
mod systems;
use systems::Render as Render;
use utility::MessageBus as MessageBus;

fn main() {
    let x = utility::Message::Quit;
    let s = utility::Message::Start;
    let q = utility::Message::Quit;
    let p = utility::Message::Pause;

    let mut mb = MessageBus{queue: Vec::new(), render_message: Render::handle_message};
    let render = Render{post_message: MessageBus::post_message};

    (render.post_message)(&mut mb, x);
    (render.post_message)(&mut mb, s);
    (render.post_message)(&mut mb, q);
    (render.post_message)(&mut mb, p);

    mb.update();
}

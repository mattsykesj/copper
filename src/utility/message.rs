#[derive(Debug)]
pub enum Message{
    Start,
    Quit,
    Pause,
    // Error {error_number: i32, error_message: &'a str},
    // Print(&'a str),
}

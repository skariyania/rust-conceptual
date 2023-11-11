trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {}
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        // &self.content
        ""
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // pub fn approve(mut self) {
    //     self.state = Some(Box::new(Published {}))
    // }

    pub fn request_review(&mut self) {}
}

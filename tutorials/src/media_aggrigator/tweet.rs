use super::summary::Summary;

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// .run tweets tutorial
pub fn run_tweets() {
    let tweet_content = Tweet {
        username: String::from("sahil"),
        content: String::from("Hello, my first tweet"),
        reply: false,
        retweet: false,
    };
    super::summary::notify(&tweet_content);
}

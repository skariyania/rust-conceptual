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

    //another way to summarize using summary traits as return type
    let resp = returns_summarizable();
    println!("{}", resp.summarize());
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Sahil"),
        content: String::from("Sahil has reviewed PR for day 7 Learnings"),
        reply: false,
        retweet: false,
    }
}

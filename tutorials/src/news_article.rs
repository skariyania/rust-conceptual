use crate::summary::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn run_news_article() {
    let news_content = NewsArticle {
        author: String::from("Sahil"),
        content: String::from("Nifty 50 below 19,300"),
        headline: String::from("Nifty was down 260.90 points or 1.34 percent at 19,281.80"),
        location: String::from("Mumbai, India"),
    };
    println!("{}", news_content.summarize());
}

use TRPL_10_02_trait::{Tweet, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("hourse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
}
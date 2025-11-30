use traits::{NewsArticle, Tweet, Summary, Summary2, Summary3, notify};
//use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people, no?"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet); // Using trait as parameter, calling notify function from traits crate
    // what will be printed when we call notify(&tweet) is the notify function plus the summarize method from the Tweet struct implementation of the Summary trait 

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.",),
    };
    println!("New article available! {}", article.summarize());
    println!("New article available! {}", article.summarize_two());
    println!("One new tweet {}", tweet.summarize_author());
    println!("Another banger tweet {}", tweet.summarize_three());
}

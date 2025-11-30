// A Summary trait that consists of the behavior provided by a summarize method

pub trait Summary {
    fn summarize(&self) -> String;
}

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

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} in {} it's {}", self.username, self.content, self.reply, self.retweet)
    }
}



// Implementing the trait Summary on the NewsArticle and Tweet types

/*#[cfg(test)]
mod tests {
    //use super::*;

    /*#[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/

    /*#[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        tweet.summarize();
    }*/
}*/

// Default Implementations
pub trait Summary2 {
    fn summarize_two (&self) -> String {
        String::from(("Read more..."))  // defining a Summary trait with a default implementation of the summarize method
    }
}

impl Summary2 for NewsArticle {
    /*fn summarize_two(&self) -> String {
        //format!("{}, by {} ({})", self.headline, self.author, self.location)
     // Implementation block should be empty when making use of a default trait   
    }*/
}

pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize_three(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // using the summarize_author method in the default implementation of summarize_three
    }
}

impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


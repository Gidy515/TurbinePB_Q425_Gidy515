// A Summary trait that consists of the behavior provided by a summarize method

use std::fmt::Debug;
use std::fmt::Display;
`

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
    fn summarize_two(&self) -> String {
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

// Trait Bound Syntax
fn notify2<T: Summary2>(item: &T) {
    println!("Breaking news! {}", item.summarize_two());
} // equivalent to notify function above but using trait bound syntax

// Specifying Multiple Trait Bounds with the + Syntax
fn notify3<T: Summary + Summary3>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//We can also specify more than one trait bound. Say we wanted notify to use display formatting as well as 
//summarize on item: we specify in the notify definition that item must implement both
//Display and Summary. We can do so using the + syntax:
//pub fn notify(item: &(impl Summary + Display)) {
//The + syntax is also valid with trait bounds on generic types:
//pub fn notify<T: Summary + Display>(item: &T) {
//With the two trait bounds specified, the body of notify can call
//summarize and use {} to format item.

// Clearer Trait Bounds with where Clauses
//fn notify4<T, U>(item1: &T, item2: &U)

/*fn some_function<T, U> (t: &T, u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{}*/

// Returning Types That Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}
    
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
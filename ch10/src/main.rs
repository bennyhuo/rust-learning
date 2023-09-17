use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T, T>
where
    T: Add + Copy,
{
    fn sum(&self) -> T::Output {
        self.x + self.y
    }
}

trait Summary {
    fn summarize(&self) -> String {
        "".to_string()
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}

fn main() {
    let news_article = NewsArticle {
        headline: "1".to_string(),
        location: "2".to_string(),
        author: "3".to_string(),
        content: "4".to_string(),
    };

    let tweet = Tweet {
        username: "bennyhuo".to_string(),
        content: "The book named 'Kotlin Metaprogramming in Action' is going to be published."
            .to_string(),
        reply: false,
        retweet: false,
    };

    notify(&tweet);

    println!("{}, {}", news_article.summarize(), tweet.summarize());

    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyzXYZ");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    };

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'l>(x: &'l str, y: &'l str) -> &'l str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

extern crate trait_test;
use trait_test::Summary;
fn main() {
    let tweet = trait_test::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"), reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    let article = trait_test::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    trait_test::notify(&article);
}

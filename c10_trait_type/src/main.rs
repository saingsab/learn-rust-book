use c10_trait_type::{NewsArticle, Summary, Tweet};
// use c10_trait_type::
fn main() {
    let tweet = Tweet {
        username: String::from("sbs"),
        content: String::from("of course, as you probably already now, people \n"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("The Cambodia Daily News"),
        location: String::from("Phnom Penh"),
        author: String::from("Ayoung"),
        content: String::from("We want to make a media aggregator library crate named aggregator that can display summaries"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    // let call the newsartile
    // println!("1 new article: {}", article.summary());

    notify(&tweet);

    // calling imp traits return
    return_summarizable();
}

pub fn notify(item: &impl Summary) {
    //shorthand of notify<T: Summary>(item: &T)
    println!("Breaking news! {}", item.summarize());
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("sbs"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

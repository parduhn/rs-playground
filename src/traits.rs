pub fn run() {
    let article = NewsArticle {
        headline: String::from("Hubs gewinnen die Stanley-Cup-Meisterschaft!"),
        location: String::from("Montreal, QC, CA"),
        author: String::from("Richard"),
        content: String::from(
            "Die Montreal Hubs sind erneut die beste \
                               Eishockeymannschaft in der NHL.",
        ),
    };

    println!("Neuer Artikel verfügbar! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("natürlich, wie du wahrscheinlich schon weißt"),
        reply: false,
        retweet: false,
    };

    println!("1 neue Kurznachricht: {}", tweet.summarize());

    let standard_article = StandardArticle {
        author: String::from("Hayo"),
        content: String::from("Wird sowieso nicht ausgegeben"),
    };

}

// default traits
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Lies mehr ...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, von {} ({})", self.headline, self.author, self.location)
    }
}

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

pub struct StandardArticle {
    pub author: String,
    pub content: String,
}

impl Summary for StandardArticle {}

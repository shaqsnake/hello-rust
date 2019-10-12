pub mod message {

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} {}", self.headline, self.author, self.location)
    //     }
    // }
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // pub fn notify(item: impl Summary) {
    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
}
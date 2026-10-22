use traits::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("hrse_ebobks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 mew post: {}", post.summarize());
}
use blog::{Post, TypePost};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // using Types

    let mut type_post = TypePost::new();

    type_post.add_text("I ate a salad for lunch today");

    let type_post = type_post.request_review();

    let type_post = type_post.approve();

    assert_eq!("I ate a salad for lunch today", type_post.content());
}

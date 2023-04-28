use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad on breakfast");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad on breakfast", post.content());
}

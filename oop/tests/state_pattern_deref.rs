use oop::blog2::Post;

#[test]
fn total_flow() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

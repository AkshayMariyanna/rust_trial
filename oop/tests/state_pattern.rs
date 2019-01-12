use oop::blog::Post;

#[test]
fn total_flow() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("!!");
    post.reject();
    post.add_text("?!");
    post.request_review();

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today?!", post.content()); 
}

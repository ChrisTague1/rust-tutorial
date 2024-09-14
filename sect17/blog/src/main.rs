use blog::Post;

// Note: There is a better way of doing this in the book that I didn't want to write out.
// It involves making it so that each state is it's own struct that returns the next struct in the
// state. The API changes because you would have to do something like
//
// let post = Post::new();
// let post = post.add_text
// let post = post.request_review
//
// This is probably better anyway
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

use state_ex::Post;

fn main() {
    
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    
    assert_eq!("I ate a salad for lunch today", post.content());
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn reject_post() {
        let mut post = Post::new();
        let msg = "Hello world";

        post.add_text(msg);
        let post = post.request_review();
        println!("PendingReviewPost: {:?}", post);

        let post = post.reject();
        println!("DrafPost: {:?}", post);
    }
}

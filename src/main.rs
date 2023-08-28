use state_ex::Post;

fn main() {
    
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    
    // assert_eq!("I ate a salad for lunch today", post.content());
}

#[cfg(test)]
mod tests {

    use state_ex::PendingReviewPost;
    use super::*;

    fn setup() -> (PendingReviewPost, &'static str) {
        let mut post = Post::new();
        let msg = "Hello world";

        post.add_text(msg);
        let post = post.request_review();
        (post, msg)
    }


    #[test]
    fn reject_post() {
        let (review_post, _) = setup();
        println!("PendingReviewPost: {:?}", review_post);

        let draft_post = review_post.reject();
        println!("DrafPost: {:?}", draft_post);
    }

    #[test]
    fn double_approval() {
        let (mut review_post, msg) = setup();
        review_post = review_post.approve().unwrap_err();
        let count = review_post.count();
        assert_eq!(count, 1);

        let post = review_post.approve().unwrap();
        assert_eq!(post.content(), msg);
    }
}

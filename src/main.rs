use state_ex::Post;

fn main() {
    
    let mut post = Post::new();


    post.add_text("Hello world");
    
    post.request_review();

    post.approve();

    println!("Post: {:?}", post);
    println!("Content: {:?}", post.content());

}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(val: u8) -> Post {
        let mut post = Post::new();
        let msg = "Hello world!";

        if val == 0 {
            return post;
        } else if val == 1 || val == 2 {
            post.add_text(msg);
            post.request_review();

            if val == 2 {
                post.approve();
            }
        } 
        post
    }

    #[test]
    fn draft_post() {
        let post = setup(0); 
        let content = post.content();
        assert_eq!(content, "");
    }

    #[test]
    fn pending_post() {
        let post = setup(1);

        let content = post.content();
        assert_eq!(content, "");

    }

    #[test] 
    fn almost_published_post_one_approval() {
        let mut post = setup(2);

        let content = post.content();
        assert_eq!(content, "");

        post.reject();
        println!("{:?}", post);
    }

    #[test]
    fn published_post_two_approvals() {
        let mut post = setup(2);
        let msg = "Hello world!";
        
        post.approve();
        let content = post.content();
        assert_eq!(msg, content);
    }

    #[test]
    fn pending_to_draft() {
        let mut post = setup(1);
        post.reject();

        let content = post.content();
        assert_eq!(content, "");
    }

    #[test]
    fn restrict_text() {
        let mut post = setup(0);
        post.request_review();
        let msg = "Hello World";

        post.add_text(msg);
        assert_eq!(post.content(), "");
    }

    #[test]
    fn add_text_only_draft() {
        let mut post = setup(2);
        post.approve();
        let msg = "Hello world!";

        let content = post.content();
       
        assert_eq!(content, msg);
    }
}

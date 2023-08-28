use std::fmt::Debug;


#[derive(Debug)]
pub struct Post {
    content: String,
}

#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { 
            content: self.content,
            count: 0
         }
    }
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
    count: u8,
}

impl PendingReviewPost {
    pub fn approve(mut self) -> Result<Post, Self> {
        if self.count < 1 {
            self.count += 1;
            Err(self)
        } else {
            Ok(Post { content: self.content, })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { 
            content: self.content,
         }
    }

    pub fn count(&self) -> u8 {
        self.count
    }
}
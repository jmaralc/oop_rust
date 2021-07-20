// we’re moving the encoding of the state to the types of the structs.
/* 
the transformations between the states are no longer encapsulated 
entirely within the Post implementation
*/
/*
our gain is that invalid states are now impossible because of the type system and 
the type checking that happens at compile time! This ensures that certain bugs, 
such as display of the content of an unpublished post, will be discovered before they 
make it to production.
*/
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    // Getter of content property
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // We can add text to a draft but there is no getter for content
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
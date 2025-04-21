// the rusty way - transitions between types
// a lot of duplication of structs
// don't always think in object oriented mindset - use type system

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    // not used as method (used with :: syntax)
    pub fn new() -> DraftPost {
        // always create a new post in the Draft state.
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
        // takes ownership of self (i.e. consumes it).
        // this way we have no lingering instances of self (DraftPost in this case)
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        // I think we use self here to indicate ownership has moved.
        Post {
            content: self.content,
        }
    }
}

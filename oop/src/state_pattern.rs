pub mod blog {
    pub struct Post {
        content: String,
        state: Option<Box<dyn State>>,
    }

    impl Post {
        pub fn new() -> Self {
            Self {
                content: String::new(),
                state: Some(Box::new(Draft {})),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.state.as_ref().unwrap().add_text(&mut self.content, text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn add_text(&self, _content: &mut String, _text: &str) {}
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview::new(2))
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn add_text(&self, content: &mut String, text: &str) {
            content.push_str(text);
        }
    }

    struct PendingReview {
        approvals_required: u32,
    }

    impl PendingReview {
        pub fn new(ar: u32) -> PendingReview {
            PendingReview { approvals_required: ar }
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            if self.approvals_required == 1 {
                Box::new(Published {})
            } else {
                Box::new(PendingReview::new(self.approvals_required - 1))
            }
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
}

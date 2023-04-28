pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        let is_editable = self.state.as_ref().unwrap().is_editable();

        if is_editable {
            self.content.push_str(text);
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
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
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn is_editable(&self) -> bool {
        false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { confirms: false })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn is_editable(&self) -> bool {
        true
    }
}

struct PendingReview {
    confirms: bool,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        let mut res: Box<dyn State> = Box::new(Published {});
        if !self.confirms {
            res = Box::new(PendingReview { confirms: true });
        }
        res
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let mut post = Post::new();
        assert_eq!("", post.content());

        post.add_text("I ate salad on breakfast");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate salad on breakfast", post.content());
    }

    #[test]
    fn must_reject() {
        let mut post = Post::new();

        post.add_text("I ate salad on breakfast");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("I ate salad on breakfast", post.content());
    }

    #[test]
    fn must_not_add_text() {
        let mut post = Post::new();

        post.add_text("I ate salad on breakfast");
        assert_eq!("", post.content());

        post.add_text(" and dinner");
        assert_eq!("", post.content());

        post.request_review();
        post.add_text(" and lanch");
        post.approve();
        post.approve();
        assert_eq!("I ate salad on breakfast and dinner", post.content());
    }
}

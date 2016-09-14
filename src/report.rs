use std::boxed::Box;
use common::Wish;
use std::rc::Rc;

#[derive(Clone)]
pub struct Well<Out, Msg> {
    adapter: Rc<Fn(Out) -> Option<Msg>>,
    pub messages: Vec<Msg>,
    pub wishes: Vec<Wish>,
}

impl<Out, Msg> Default for Well<Out, Msg> {
    fn default() -> Self {
        Well::new(|_| None)
    }
}

impl<Out, Msg> Well<Out, Msg> {
    pub fn new<F>(adapter: F) -> Self where F: Fn(Out) -> Option<Msg> + 'static {
        Well {
            adapter: Rc::new(adapter),
            messages: vec![],
            wishes: vec![],
        }
    }
    pub fn add_out(&mut self, out: Out) {
        if let Some(message) = self.adapter.as_ref()(out) {
            self.messages.push(message)
        }
    }
    pub fn add_wish(&mut self, wish: Wish) {
        self.wishes.push(wish)
    }
}
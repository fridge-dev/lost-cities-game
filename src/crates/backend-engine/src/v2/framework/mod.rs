pub mod prng;
pub mod shuffler;

use std::collections::HashMap;

pub struct ClientOut {

}

impl ClientOut {

    // TODO is there a difference?
    pub fn send<M: Into<prost::Message>>(&self, message: M) {
        unimplemented!()
    }

    pub fn send2(&self, message: impl Into<prost::Message>) {
        unimplemented!()
    }

    pub fn send_err(&self, message: &str) {

    }
}

pub struct Holder<T> (Option<T>);

impl<T> Holder<T> {
    pub fn new(item: T) -> Self {
        Holder(Some(item))
    }

    pub fn take(&mut self) -> T {
        self.0.take().expect("Invalid state: Holder.take() called when it was empty")
    }

    pub fn put(&mut self, item: T) {
        if self.0.is_some() {
            panic!("Invalid state: Holder.put() called when it was full");
        }
        self.0.replace(item);
    }

    pub fn peek(&self) -> &T {
        &self.0.expect("Invalid state: Holder.take() called when it was empty")
    }
}

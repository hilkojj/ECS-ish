use crate::ecs::World;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AfterUpdate {
    pub(crate) functions: Arc<Mutex<Vec<Box<Fn(&mut World) -> () + Sync + Send>>>>,
}

impl AfterUpdate {

    pub(crate) fn new() -> Self {
        Self {
            functions: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn exec<T>(&self, function: T)
    where
        T: Fn(&mut World) -> () + Send + Sync + 'static,
    {
        self.functions.lock().unwrap().push(Box::new(function));
    }
}

use crate::ecs::{EntityId, World};
use std::sync::{Arc, Mutex};

type Func = Fn(&mut World) -> () + Sync + Send;

#[derive(Clone)]
pub struct AfterUpdate {
    pub(crate) functions: Arc<Mutex<Vec<Box<Func>>>>,
}

impl AfterUpdate {
    pub(crate) fn new() -> Self {
        Self {
            functions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn exec<T>(&self, function: T)
    where
        T: Fn(&mut World) -> () + Send + Sync + 'static,
    {
        self.functions.lock().unwrap().push(Box::new(function));
    }

    pub fn remove_component<T>(&self, id: EntityId)
    where
        T: 'static,
    {
        self.exec(move |world| {
            world.remove_component::<T>(id);
        });
    }

    pub fn add_clonable_component<T>(&self, id: EntityId, component: T)
    where
        T: 'static + Send + Sync + Clone,
    {
        self.exec(move |world| {
            world.add_component(id, component.clone());
        });
    }
}

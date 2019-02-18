
use std::marker::PhantomData;

#[derive(Clone)]
pub struct ComponentType<T> {

    pub(crate) index: usize,
    i_love_pizza: PhantomData<T>

}

impl<T> ComponentType<T> {

    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
            i_love_pizza: PhantomData
        }
    }

}

use std::{
    collections::HashMap,
    rc::{Rc, Weak},
}

#[derive(Clone)]
pub struct ScopeMap<K, V> {
    inner: Rc<Inner<K, V>>,
}


#[derive(Debug)]
struct Inner<K, V> {
    parent: Option<ScopeMap<K, V>>,
    scope: HashMap<K, Option<V>>,
}

impl<K, V> ScopeMap<K, V> {
    pub fn new() -> Self {
        let inner = Rc::new(Inner {
            parent: None,
            scope: HashMap::new(),
        });
        ScopeMap { inner, }
    }

    pub fn child(&self) -> Self {
        let inner = Rc::new(Inner {
            parent: self.clone(),
            scope: HashMap::new(),
        });
        ScopeMap { inner, }
    }
}

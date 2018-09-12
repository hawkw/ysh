use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
    rc::Rc,
    cell::{RefCell, Ref},
};

#[derive(Debug, Clone)]
pub struct ScopeMap<K: Hash + Eq, V> {
    inner: Rc<Inner<K, V>>,
}


#[derive(Debug)]
struct Inner<K: Hash + Eq, V> {
    parent: Option<ScopeMap<K, V>>,
    // if the value is `None`, it has been "whited out" at this level.
    scope: HashMap<K, Option<V>>,
}

impl<K, V> Inner<K, V>
where
    K: Hash + Eq,
{
    fn new(parent: Option<ScopeMap<K,V>>) -> Rc<RefCell<Self>> {
        let inner = Self {
            parent,
            scope: HashMap::new(),
        };
        Rc::new(RefCell::new(inner))
    }
}

impl<K, V> ScopeMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        let inner = Inner::new(None);
        ScopeMap { inner, }
    }

    pub fn child(&self) -> Self {
        let me = ScopeMap { inner: Rc::clone(&self.inner) };
        let inner = Inner::new(Some(me));
        ScopeMap { inner, }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let this: Ref<Inner<K, V>> = RefCell::borrow(self.inner.as_ref());
        match this.scope.get(k) {
            Some(None) => None,
            Some(Some(v)) => Some(v),
            None => {
                let parent = this.parent.as_ref();
                parent.and_then(|parent| parent.get(k))
            },

        }
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        // this one requires a little bit of a dance in order to fake
        // "mutating" a value that exists in a higher layer.
        let mut this = self.inner.borrow_mut();
        if this.scope.contains_key(k) {
            this.scope.get_mut(k).and_then(|x| x.as_mut())
        } else {
            unimplemented!()
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.borrow_mut().scope.insert(k, Some(v)).and_then(|x| x)
    }

}

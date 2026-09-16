use std::sync::atomic::{AtomicU32, Ordering};

use flutter_rust_bridge::DartFnFuture;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct FxEvent<T>(RwLock<Option<Box<dyn Fn(T) -> DartFnFuture<()> + Send + Sync + 'static>>>);

impl<T> FxEvent<T> {
    pub fn invoke(&self, param: T) {
        if let Some(it) = &*self.0.blocking_read() {
            crate::spawn((it)(param));
        }
    }

    pub fn set(&self, callback: impl Fn(T) -> DartFnFuture<()> + Send + Sync + 'static) {
        *self.0.blocking_write() = Some(Box::new(callback));
    }

    pub fn clear(&self) {
        *self.0.blocking_write() = None;
    }
}

pub struct FxVecEvent<T: Clone> {
    next: AtomicU32,
    subs: RwLock<Vec<FxEventSubscription<T>>>,
}

impl<T: Clone> Default for FxVecEvent<T> {
    fn default() -> Self {
        Self {
            next: Default::default(),
            subs: Default::default(),
        }
    }
}

struct FxEventSubscription<T: Clone> {
    id: u32,
    callback: Box<dyn Fn(T) -> DartFnFuture<()> + Send + Sync + 'static>,
}

impl<T: Clone + Send + Sync + 'static> FxVecEvent<T> {
    pub fn invoke(&self, param: T) {
        let subs = &*self.subs.blocking_read();
        for sub in subs {
            crate::spawn((sub.callback)(param.clone()));
        }
    }

    pub fn add(&self, callback: impl Fn(T) -> DartFnFuture<()> + Send + Sync + 'static) -> u32 {
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        self.subs.blocking_write().push(FxEventSubscription {
            id,
            callback: Box::new(callback),
        });
        id
    }

    pub fn remove(&self, id: u32) {
        let mut listeners = self.subs.blocking_write();
        listeners.retain(|it| it.id != id);
    }

    pub fn clear(&self) {
        let mut listeners = self.subs.blocking_write();
        listeners.clear();
    }
}

/// This event-registry is based on Type-Driven API Design in Rust.
/// see: https://willcrichton.net/rust-api-type-patterns/registries.html
/// Only major change is the support of dependency injection via a single Arc.
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type Dependency = Arc<dyn Any + Send + Sync>;

type EventHandler<E> =
    Box<dyn Fn(Dependency, E) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

struct EventListener<E> {
    dependency: Dependency,
    handler: EventHandler<E>,
}
pub struct EventDispatcher {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            handlers: HashMap::new(),
        }
    }

    pub fn add_event_listener<E, D, F, Fut>(&mut self, dependency: Arc<D>, handler: F)
    where
        E: 'static + Send + Sync,
        D: 'static + Send + Sync,
        F: Fn(Arc<D>, E) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let type_id = TypeId::of::<E>();
        println!("adding event listener: {:?}...", type_id);

        let listeners = self
            .handlers
            .entry(type_id)
            .or_insert_with(|| Box::new(Vec::<EventListener<E>>::new()))
            .downcast_mut::<Vec<EventListener<E>>>()
            .expect("Could not downcast");

        listeners.push(EventListener {
            dependency: Arc::new(dependency) as Arc<dyn Any + Send + Sync>,
            handler: Box::new(move |dep, event| {
                let dep = dep
                    .downcast_ref::<Arc<D>>()
                    .expect("Dependency type mismatch");

                Box::pin(handler(dep.clone(), event))
            }),
        });

        println!("Added");
    }

    pub async fn trigger<E: 'static + Send + Sync + Clone>(&self, event: E) {
        let type_id = TypeId::of::<E>();
        println!("retrieving event listeners for: {:?}...", type_id);

        if let Some(listeners) = self
            .handlers
            .get(&type_id)
            .and_then(|box_any| box_any.downcast_ref::<Vec<EventListener<E>>>())
        {
            println!("there were some listeners");
            let mut tasks = vec![];
            for listener in listeners {
                let handler = &listener.handler;
                let fut = handler(listener.dependency.clone(), event.clone());
                let task = tokio::task::spawn(async move {
                    fut.await;
                });
                tasks.push(task);
            }
            let results = futures::future::join_all(tasks).await;
            for result in results {
                if let Err(e) = result {
                    eprintln!("Task failed: {:?}", e)
                }
            }
        } else {
            println!("No listeners found for {:?}", type_id);
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

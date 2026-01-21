// TODO: Refrence where you copied it from the http crates

use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn type_name(&self) -> &'static str;
}

impl<T: Clone + Send + Sync + 'static> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}

impl Clone for Box<dyn AnyClone + Send + Sync> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

type MapValue = Box<HashMap<TypeId, Box<dyn AnyClone + Send + Sync>, BuildHasherDefault<IdHasher>>>;

#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

// This hold all the state we wanna share between our Context and it shold be easy to clone
// Complete this api later add test and reference to the main source code
#[derive(Clone)]
pub struct Map(Option<MapValue>);

impl Map {
    /// Create an empty `Map`.
    #[inline]
    pub fn new() -> Map {
        Map(None)
    }

    pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.0
            .get_or_insert_with(Box::default)
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| {
                boxed
                    .into_any()
                    .downcast()
                    .ok()
                    .map(|boxed| *boxed)
            })
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.0
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| (**boxed).as_any().downcast_ref())
    }

    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.0
            .as_mut()
            .and_then(|map| map.get_mut(&TypeId::of::<T>()))
            .and_then(|boxed| (**boxed).as_any_mut().downcast_mut())
    }

    pub fn get_or_insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }

    pub fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
        let out = self
            .0
            .get_or_insert_with(Box::default)
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(f()));
        (**out)
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    pub fn get_or_insert_default<T: Default + Clone + Send + Sync + 'static>(&mut self) -> &mut T {
        self.get_or_insert_with(T::default)
    }

    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.0
            .as_mut()
            .and_then(|map| map.remove(&TypeId::of::<T>()))
            .and_then(|boxed| {
                boxed
                    .into_any()
                    .downcast()
                    .ok()
                    .map(|boxed| *boxed)
            })
    }

    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut map) = self.0 {
            map.clear();
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0
            .as_ref()
            .map_or(true, |map| map.is_empty())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0
            .as_ref()
            .map_or(0, |map| map.len())
    }
}

impl Default for Map {
    fn default() -> Self {
        Self(None)
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement this later
        f.debug_struct("Map").finish()
    }
}

fn downcast_owned<T: 'static>(boxed: Box<dyn AnyClone + Send + Sync>) -> Option<T> {
    // println!("{:?}", boxed);
    boxed
        .into_any()
        .downcast::<T>()
        .ok()
        .map(|b| *b)
}

fn downcast_ref<'a, T: 'static>(boxed: &'a Box<dyn AnyClone + Send + Sync>) -> Option<&'a T> {
    (**boxed).as_any().downcast_ref::<T>()
}

fn downcast_mut<'a, T: 'static>(boxed: &'a mut Box<dyn AnyClone + Send + Sync + 'static>) -> Option<&'a mut T> {
    (**boxed)
        .as_any_mut()
        .downcast_mut::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_map() {
        // This macro inserts a list of handler types into the map and verifies
        // that each one can be retrieved correctly. It ensures that insertion
        // and lookup both behave as expected.
        macro_rules! test_map_macro {
            ($map:expr, $($name:ident),*) => {
                $(
                    #[derive(Debug, Clone, PartialEq)]
                    struct $name;
                    $map.insert($name);

                    assert_eq!($map.get::<$name>(), Some(&$name), "Failed to retrieve {} from the map", stringify!($name));
                )*
            };
        }

        let mut map = Map::new();
        test_map_macro!(map, A, B, C, D, E);
        assert_eq!(map.len(), 5);

        let mut new_map = map.clone();
        // Insert additional handlers into a cloned map to ensure cloning
        // produces an independent copy that can evolve separately.
        test_map_macro!(new_map, F, G, H, I, J, K, L);
        assert_eq!(map.len(), 5, "Original map should still contain 5 entries");

        // New map should reflect all inserted handlers.
        assert_eq!(new_map.len(), 12, "Cloned map should contain all original and newly added handlers");
    }
}

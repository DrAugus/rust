use std::fmt;
use std::hash::Hash;

pub trait CacheableItem: Clone + Default + fmt::Debug {
    type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    fn is_null(&self) -> bool;
}

trait Container<A, B> {
    fn contains(&self, a: A, b: B) -> bool;
}

fn difference<A, B, C>(container: &C) -> i32 where C: Container<A, B> { 1 }

//Generic refactor
trait ContainerGeneric {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

fn difference_generic<C: ContainerGeneric>(container: &C) {}

#[cfg(test)]
mod tests {
    #[test]

    fn generic() {

    }
}


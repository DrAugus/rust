#[allow(dead_code)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn _create_empty_list() -> List {
    List::Nil
}

pub fn _create_non_empty_list() -> List {
    List::Cons(1, Box::from(List::Nil))
}


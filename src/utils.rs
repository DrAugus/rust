pub(crate) fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub(crate) fn add<T: std::ops::Add<Output=T>>(i: T, j: T) -> T {
    i + j
}

pub(crate) struct Point<T: std::ops::Add<T, Output=T>> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils() {
        let arr_find_largest = [1, 2, 3, 4, 9, 6, 1];
        assert_eq!(largest(&arr_find_largest), 9);
    }
}

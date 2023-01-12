#[allow(dead_code)]
pub(crate) fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
#[allow(dead_code)]
pub(crate) fn add<T: std::ops::Add<Output=T>>(i: T, j: T) -> T {
    i + j
}
#[allow(dead_code)]
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


pub fn _factorial(num: u64) -> u64 {
    (1..=num).product()
}

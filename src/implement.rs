use crate::utils::Point;
use crate::utils::add;

impl<T: std::ops::Add<T, Output=T>> std::ops::Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: add(self.x, p.x),
            y: add(self.y, p.y),
            z: add(self.z, p.z),
        }
    }
}
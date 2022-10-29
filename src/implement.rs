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


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        dbg!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        dbg!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        dbg!("*waving arms furiously*");
    }
}

trait Animal {
    fn babe_name() -> String;
}

struct Cat;

impl Cat {
    fn babe_name() -> String {
        "babe".to_string()
    }
}

impl Animal for Cat {
    fn babe_name() -> String {
        "meow".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn implements() {
        let person_fly = Human;
        Pilot::fly(&person_fly);
        Wizard::fly(&person_fly);
        person_fly.fly();

        // 完全限定语法 定义  <Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A babe cat is called a {}", <Cat as Animal>::babe_name());
    }
}

// 各种特性


struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// Self 指代被实现方法的结构体类型，self 指代此类型的实例
impl Circle {
    fn _new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    fn _area(&self) -> f64 { std::f64::consts::PI * (self.radius * self.radius) }
}



pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}


pub struct Screen {
    // 动态数组，类型是 Draw 特征对象：Box<dyn Draw>，
    // 任何实现了 Draw 特征的类型，都可以存放其中。
    // 当使用特征对象时，Rust 必须使用动态分发
    // 关于【特征对象的动态分发】还得仔细研究
    // https://course.rs/basic/trait/trait-object.html#%E7%89%B9%E5%BE%81%E5%AF%B9%E8%B1%A1%E7%9A%84%E5%8A%A8%E6%80%81%E5%88%86%E5%8F%91
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 泛型实现
pub struct GenericScreen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> GenericScreen<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl Draw for u8 {
    fn draw(&self) {
        println!("u8 {}", *self);
    }
}

impl Draw for f64 {
    fn draw(&self) {
        println!("f64 {}", *self);
    }
}

fn draw1(x: Box<dyn Draw>) { x.draw(); }

fn draw2(x: &dyn Draw) { x.draw(); }

pub(crate) fn show_screen() {

    let draw_x = 1.1f64;
    let draw_y = 8u8;
    draw1(Box::new(draw_x));
    draw1(Box::new(draw_y));
    draw2(&draw_x);
    draw2(&draw_y);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

}



// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: std::ops::Add<T, Output=T>> {
    x: T,
    y: T,
    z: T,
}

struct Color(u8, u8, u8);

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

impl Direction {
    const DIRECTION: [Self; 8] = [
        Direction::East,
        Direction::West,
        Direction::North,
        Direction::South,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthEast,
        Direction::NorthWest,
    ];
}

fn _not_invoke() {
    for i in Direction::DIRECTION {
        print!("{:?}", i);
    }
}

enum Action {
    Say(String),
    MoveTo(Point<f32>, Point<f32>),
    ChangeColor(Color),
    Loading(bool),
}

fn op_action(op: Action) {
    match op {
        Action::Say(s) => println!("{}", s),
        Action::MoveTo(x, y) => {
            println!("point from ({:?}) move to ({:?})", x, y);
        }
        Action::ChangeColor(c) => {
            println!("change color into (r:{}, g:{}, b:{}))",
                     c.0, c.1, c.2);
        }
        _ => {
            println!("loading");
        }
    }
}


enum Message {
    ChangeColor((u8, u8, u8)),
    Echo(String),
    Move(Point<i32>),
    Quit,
}


struct State {
    color: (u8, u8, u8),
    position: Point<i32>,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point<i32>) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(c) => self.change_color(c),
            Message::Quit => self.quit(),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action() {
        let say_it = Action::Say("unwrap".to_string());
        // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
        if let Action::Say(s) = say_it {
            println!("hit! {}", s);
        }
        // matches 暂不展开
    
        let actions = [
            Action::Say("hi".to_string()),
            Action::MoveTo(Point { x: 2.0, y: 3.0, z: 1.0 },
                           Point { x: 2.0, y: 3.0, z: 1.0 }),
            Action::ChangeColor(Color(2, 3, 4)),
            Action::Loading(true),
        ];
        for action in actions {
            op_action(action);
        }
    }


    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0, z: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15, z: 20 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}

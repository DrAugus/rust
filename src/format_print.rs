// 本页代码大多来自 https://course.rs/basic/formatted-output.html
// 当作字典查询使用

fn _pos() {
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112
}

fn _name() {
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
}


mod format {
    mod width {
        // 字符串填充
        fn _string_fill() {
            //-----------------------------------
            // 以下全部输出 "Hello x    !"
            // 为"x"后面填充空格，补齐宽度5
            println!("Hello {:5}!", "x");
            // 使用参数5来指定宽度
            println!("Hello {:1$}!", "x", 5);
            // 使用x作为占位符输出内容，同时使用5作为宽度
            println!("Hello {1:0$}!", 5, "x");
            // 使用有名称的参数作为宽度
            println!("Hello {:width$}!", "x", width = 5);
            //-----------------------------------

            // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
            println!("Hello {:1$}!{}", "x", 5);
        }

        // 数字填充:符号和 0
        fn _number_fill() {
            // 宽度是5 => Hello     5!
            println!("Hello {:5}!", 5);
            // 显式的输出正号 => Hello +5!
            println!("Hello {:+}!", 5);
            // 宽度5，使用0进行填充 => Hello 00005!
            println!("Hello {:05}!", 5);
            // 负号也要占用一位宽度 => Hello -0005!
            println!("Hello {:05}!", -5);
        }

        fn _align() {
            // 以下全部都会补齐5个字符的长度
            // 左对齐 => Hello x    !
            println!("Hello {:<5}!", "x");
            // 右对齐 => Hello     x!
            println!("Hello {:>5}!", "x");
            // 居中对齐 => Hello   x  !
            println!("Hello {:^5}!", "x");

            // 对齐并使用指定符号填充 => Hello x&&&&!
            // 指定符号填充的前提条件是必须有对齐字符
            println!("Hello {:&<5}!", "x");
        }
    }

    fn _precision() {
        let v = 3.1415926;
        // 保留小数点后两位 => 3.14
        println!("{:.2}", v);
        // 带符号保留小数点后两位 => +3.14
        println!("{:+.2}", v);
        // 不带小数 => 3
        println!("{:.0}", v);
        // 通过参数来设定精度 => 3.1416，相当于{:.4}
        println!("{:.1$}", v, 4);

        let s = "hi我是Sunface孙飞";
        // 保留字符串前三个字符 => hi我
        println!("{:.3}", s);
        // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
        println!("Hello {:.*}!", 3, "abcdefg");
    }

    fn _binary() {
        // 二进制 => 0b11011!
        println!("{:#b}!", 27);
        // 八进制 => 0o33!
        println!("{:#o}!", 27);
        // 十进制 => 27!
        println!("{}!", 27);
        // 小写十六进制 => 0x1b!
        println!("{:#x}!", 27);
        // 大写十六进制 => 0x1B!
        println!("{:#X}!", 27);

        // 不带前缀的十六进制 => 1b!
        println!("{:x}!", 27);

        // 使用0填充二进制，宽度为10 => 0b00011011!
        println!("{:#010b}!", 27);
    }

    // 指数
    fn _index() {
        println!("{:2e}", 1000000000); // => 1e9
        println!("{:2E}", 1000000000); // => 1E9
    }

    fn _address() {
        let v = vec![1, 2, 3];
        println!("{:p}", v.as_ptr()) // => 0x600002324050
    }
}


pub(crate) fn _format_print() {}
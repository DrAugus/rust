# rust

> following [Rust Course](https://course.rs/)

## Preliminary exploration

1. build and run: `cargo run`
2. build: `cargo build`
3. run: `./target/debug/rust`
4. release build and run: `cargo run --release`
5. release build: `cargo build --release`
6. `cargo check`: 当项目大了后，`cargo run` 和 `cargo build` 不可避免的会变慢。此时，使用 `cargo check`
   快速的检查一下代码能否编译通过。因此该命令速度会非常快，能节省大量的编译时间。


- `Cargo.toml` 是 cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。
- `Cargo.lock` 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它，只需要对着 Cargo.toml 文件撸就行了。
  > 什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，就上传 Cargo.lock，如果是一个依赖库项目，那么请把它添加到 .gitignore 中

## Basic

### Self 与 self

self指代的就是当前的实例对象，也就是 button.draw() 中的 button 实例，Self 则指代的是 Button 类型。

不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：

* 方法的返回类型不能是 `Self`
* 方法没有任何泛型参数
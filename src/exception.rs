use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
#[allow(dead_code)]
fn op_file() {
    let f = File::open("h.txt");

    let _f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("h.txt") {
                Ok(fc) => match fs::remove_file("h.txt") {
                    Ok(_okk) => {
                        println!("remove success");
                        fc
                    }
                    Err(er) => panic!("couldn't remove {:?}", er),
                },
                Err(e) => panic!("Create failed: {:?}", e),
            },
            other => panic!("Open failed: {:?}", other),
        },
    };
}
#[allow(dead_code)]
// 传播错误的 迭代改进
fn read_username_from_file_1() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("data/text.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}
#[allow(dead_code)]
fn read_username_from_file_2() -> Result<String, io::Error> {
    let f = File::open("data/text.txt");
    let mut s = String::new();
    f?.read_to_string(&mut s)?;
    dbg!(&s);
    Ok(s)
}
#[allow(dead_code)]
fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("data/text.txt")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exception() {
    op_file();
    read_username_from_file_1().expect("panic message");
    read_username_from_file_2().expect("TODO: panic message");
    read_username_from_file_3().expect("TODO: panic message");
}}
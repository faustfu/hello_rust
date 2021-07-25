use std::string::FromUtf8Error;

fn run() -> Result<(), FromUtf8Error> {
    let s1 = "123"; // 在 DS 建立
    let obj = String::from("123"); // 在 Heap 建立
    let s2 = obj.as_str(); // 取得底層切片
    let arr = String::from_utf8(vec![0x0031, 0x0032, 0x0033])?; //
    let s3 = arr.as_str();

    println!("&str in data segment is {}", s1);
    println!("&str in heap is {}", s2);
    println!("&str in stack is {}", s3);
    Ok(())
}

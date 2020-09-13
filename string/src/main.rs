fn main() {
    let s = "hello rust world!";

    // 部分文字列を取得
    // [n..m]という書き方をした場合、
    // 文字aは、n <= a < m という範囲になる
    let hello = &s[0..5];
    println!("hello is {}", hello);

    // 長さを取得
    let len = s.len();
    println!("s.len is {}", len);

    // 文字列を作成する
    let mut str = String::new();
    str.push_str("hello ");
    str.push_str("world");
    println!("str is {}", str);

    // 文字列を扱う場合、&str型と&String型の2種類がある
    // &str型が固定文字列
    // &String型が可変文字列を扱う
    // 予め、文字列を設定した&String型の変数を作成する場合は、str::to_string関数化String::from関数を使う
    let str1 = "hello world".to_string();
    println!("str1 is {}", str1);

    let str2 = String::from("hello world");
    println!("str2 is {}", str2);
}

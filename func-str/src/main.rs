fn main() {
    let ret = str_param_and_return("rust");
    println!("ret is {}", ret);
}

// 文字列を返す関数
fn str_param_and_return(s: &str) -> String {
    let ret = format!("hello {} world.", s);
    ret
}

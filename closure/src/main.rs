/** 
 * クロージャでは２つの「|」で囲まれた部分がクロージャの引数になります
 * クロージャの場合、引数に型は必要ありません
*/
fn main() {
    let num = 10;
    let add_one = |x| { num + x };

    let ans = add_one(1);
    println!("ans is {}", ans);
}

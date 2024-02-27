fn main() {
    let mut text = String::from("aaaa bbbb");

    let first = first_word(&text);

    // println で first を使っており借用状態なのでもとの変数を変更することができない
    // text.clear();

    println!("{}", first);

    // first は println 以降は使用をしていないので借用状態から解放されるので変更が可能になる
    text.clear();
}

fn first_word(s: &str) -> &str {
    let data = s.as_bytes();
    for (i, &c) in data.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

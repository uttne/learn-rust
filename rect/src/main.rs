// {:?}で構造体の中身をみえるようにするための注釈
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 関連関数として Rectangle::square で実行できる関数を impl に書くことができる
    // self を書かない場合そうなる
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// impl は複数の場所で定義可能
// -> つまり既存の構造体のメソッドを拡張できるということ?
impl Rectangle {
    fn area2(&self) -> u32 {
        self.width * self.height
    }
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

fn main() {
    let rec = Rectangle {
        width: 2,
        height: 3,
    };

    println!("{:?}", rec);
    // #をつけると改行して複数のフィールドの値を見やすくしてくれる
    println!("{:#?}", rec);

    let ans = area(&rec);
    println!("area : {}", ans);

    println!("area : {}", rec.area());
}

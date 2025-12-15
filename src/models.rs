// 構造体を公開する (pub struct)
#[derive(Debug)]
pub struct Somecode {
    a: i32,
    b: i32,
}

#[derive(Debug)]
pub struct Othercode {
    x: f64,
    y: f64,
}

// トレイトも公開する (pub trait)
pub trait Mydisplay {
    fn display(&self) -> String;
}

// 実装ブロック (impl) 自体に pub は要りませんが、
// 中の関数 (fn) には pub が必要です！
impl Somecode {
    pub fn new(a: i32, b: i32) -> Self {
        Somecode { a, b }
    }
}

impl Othercode {
    pub fn new(x: f64, y: f64) -> Self {
        Othercode { x, y }
    }
}

// トレイトの実装は、トレイトと構造体が pub なら自動的に見えます
impl Mydisplay for Somecode {
    fn display(&self) -> String {
        format!("Somecode(a: {}, b: {})", self.a, self.b)
    }
}

impl Mydisplay for Othercode {
    fn display(&self) -> String {
        format!("Othercode(x: {}, y: {})", self.x, self.y)
    }
}

// 引数の型指定も、同じファイル内なので models:: は不要
pub fn print_data(item: &impl Mydisplay) {
    println!("{}", item.display());
}
//ジェネリクスを使用した書き方
// pub fn print_data<T: Mydisplay>(item: &T) {
//     println!("{}", item.display());
// }
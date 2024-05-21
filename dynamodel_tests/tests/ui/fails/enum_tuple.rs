use dynamodel::Dynamodel;

#[derive(Dynamodel)]
enum Tuple {
    Str(String, String),
    Int(i64, i64),
}

fn main() {}

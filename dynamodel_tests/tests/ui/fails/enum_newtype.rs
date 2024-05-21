use dynamodel::Dynamodel;

#[derive(Dynamodel)]
enum NewType {
    Str(String),
    Int(i64),
}

fn main() {}

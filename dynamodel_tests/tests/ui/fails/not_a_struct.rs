use dynamodel::Dynamodel;

#[derive(Dynamodel)]
enum NotAStruct {
    VariantA(i32),
    VariantB(String),
    VariantC(Vec<String>),
}

fn main() {}

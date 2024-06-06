use dynamodel::{AttributeValueConvertible, Dynamodel};

#[derive(Dynamodel)]
struct ExampleStruct<T: AttributeValueConvertible> {
    attr: T,
}

#[derive(Dynamodel)]
enum ExampleEnum<T: AttributeValueConvertible> {
    A { attr: T },
    B { attr: String },
}

fn main() {}

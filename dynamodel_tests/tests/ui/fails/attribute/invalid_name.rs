use dynamodel::Dynamodel;

#[derive(Dynamodel)]
struct InvalidAttributeName {
    #[dynamodel(foo(path = "unknown"))]
    bytes: Vec<u8>,
}

fn main() {}

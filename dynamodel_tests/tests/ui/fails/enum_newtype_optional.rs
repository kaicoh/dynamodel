use dynamodel::Dynamodel;

#[derive(Dynamodel)]
enum NewType {
    Str(Option<String>),
}

fn main() {}

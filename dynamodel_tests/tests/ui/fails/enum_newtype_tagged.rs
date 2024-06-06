use dynamodel::Dynamodel;

#[derive(Dynamodel)]
#[dynamodel(tag = "type")]
enum NewType {
    Str(String),
}

fn main() {}

use dynamodel::Dynamodel;

#[derive(Dynamodel)]
struct VideoComment {
    #[dynamodel(try_from = "foo", try_from_item = "bar")]
    timestamp: String
}

fn main() {}

use dynamodel::Dynamodel;

#[derive(Dynamodel)]
#[dynamodel(extra = "unknown")]
struct Video {
    id: String,
    author: String,
    uploaded_at: String,
}

fn main() {}

use dynamodel::Dynamodel;

#[derive(Dynamodel)]
#[dynamodel(table_key = "unknown")]
struct Video {
    id: String,
    author: String,
    uploaded_at: String,
}

fn main() {}

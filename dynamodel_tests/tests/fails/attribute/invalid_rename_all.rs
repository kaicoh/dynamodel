use dynamodel::Dynamodel;

#[derive(Dynamodel)]
#[dynamodel(rename_all = "unknown")]
struct InvalidRename {
    bytes: Vec<u8>,
}

fn main() {}

// Data Structures
enum Block {
    Heading {
        level: u8,
        text: Vec<Inline>,
    },
    List {
        ordered: bool,
        text: Vec<Vec<Inline>>,
    },
    Paragraph {
        text: Vec<Inline>,
    },
}

enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Link {
        link_text: Vec<Inline>,
        link_url: String,
    },
}

struct Document {
    blocks: Vec<Block>,
}

// Orchistration
fn main() {
    println!("Hello, world!");
}

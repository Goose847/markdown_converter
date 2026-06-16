enum Block {
    Heading,
    Italic,
    Bold,
    BulletList,
    OrderedList,
    Link,
    Paragraph,
}

struct Heading {
    level: u8,
    text: String,
}
struct List {
    ordered: bool,
    text: String,
}
struct Emphasis {
    bold: bool,
    text: String,
}
struct Link {
    text: String,
}

fn main() {
    println!("Hello, world!");
}

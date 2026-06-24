use std::fmt;

use clap::Parser;

//Commnd Line Interface
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    input_file: String,
    output_file: Option<String>,
}

// Error Hanldling
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    ParseError { line_number: u32, message: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::ParseError {
                line_number,
                message,
            } => write!(f, "Parse error on line {}: {}", line_number, message),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}
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
    let cli = Cli::parse();
    println!("{0}", cli.input_file);
}

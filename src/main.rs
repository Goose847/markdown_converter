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
impl Document {
    fn new() -> Document {
        Document {
            blocks : Vec::new(),
        }
    }
}

// Parser Logic
fn get_rest_of_line(line: &str) -> &str {
    line.split_once(char::is_whitespace)
        .map(|(_, rest)| rest.trim_start())
        .unwrap_or("")
}

fn inline_parser(line: &str) -> Result<Inline, AppError> {
    todo!()
}

fn parse(input: &str) -> Result<Document, AppError> {
    let document = Document::new();
    let doc_lines = input.lines();
    for line in doc_lines {
        if let Some((_, character)) = line.char_indices().find(|(_, c)| !c.is_whitespace()){
            match character{
                '#' => {
                    let level = line
                        .split_whitespace()
                        .next()
                        .map_or(0, |word| word.chars().count());
                    
                    let raw_text = get_rest_of_line(&line);

                    document.blocks.push(Block::Heading{
                        level: level.try_into().unwrap(), text,
                    })
                },
                '-' => {
                    todo!()
                    // Figure out if it is the beginning of a list or part of an existing list
                    // extract text
                    // check inlines
                    // put into block for each item 
                    // push to Document

                },
                _ => todo!(),
            } 
        }
        
    }
    Ok(document)
}

//html formatting

// Orchistration
fn main() {
    let cli = Cli::parse();
    let file = std::fs::read_to_string(cli.input_file)
        .expect("shoulda been a file here -- handle better later");
    let doc = parse(&file);
}

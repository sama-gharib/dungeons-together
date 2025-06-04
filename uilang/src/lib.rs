//! # Introduction
//! This crate exports the `uilang!` procedural macro. It enables to create
//! UIs with the [`desi-ui`](https://github.com/sama-gharib/dungeons-together/tree/main/desi-ui/) crate
//! with a simple markup language. 
//! <div class="warning">
//!     <p > Currently, the <code>uilang!</code> macro only checks your UI code syntax. </p>
//! </div>
//! 
//! # Example
//! 
//! ```
//! use uilang::uilang;
//! use macroquad::prelude::*; 
//! use desi_ui::*;
//!
//! # async fn exemple() {
//! let mut ui = uilang!(
//!     <Frame>
//!         primary: "BLUE"
//!         <Label>
//!             scale: "(0.8, 0.3)"
//!         </Label>
//!         <Button>
//!             center: "(0.0, 0.4)"
//!             scale: "(0.2, 0.2)"
//!             <Label>
//!                 primary: "WHITE"
//!             </Label>
//!         </Button>
//!     </Frame>
//! );
//! 
//! ui.update_absolutes(Layout { center: vec2(400.0, 300.0), scale: vec2(800.0, 600.0) });
//! 
//! loop {
//!     clear_background(RED);
//!     
//!     ui.draw();
//!     ui.get_activations();
//!     
//!     next_frame().await;
//! }
//! # }
//! ```
//! # Language specifications
//! The `uilang` language is defined by the following context-free grammar in BNF : 
//! ```text
//! 
//! <ui>          ::= <opening> <parameters> <children> <closing>
//! <opening>     ::= "<" <identifier> ">"
//! <closing>     ::= "</" <identifier> ">"
//! <parameters>  ::= <parameter> <parameters> | ""
//! <parameter>   ::= <identifier> ":" <string-literal>
//! <children>    ::= <ui>  <children> | ""
//! ```
//! Where `<identifier>` and `<string-literal>` are Rust tokens. 
//! Moreover, any `<opening>` and `<closing>` identifiers must match when in the same `<ui>`.
//! Newlines and indentations are ignored.


use proc_macro::{ Literal, TokenStream, TokenTree };
use desi_ui::{ WidgetData };

/// The list of valid parameters
#[derive(Debug)]
enum Parameter {
    Id,
    Position,
    Size,
    Text,
    Primary,
    Secondary
}


impl From<&str> for Parameter {
    /// Also checks the validity of `s`    
    fn from(s: &str) -> Self {
        match s {
            "id" => Self::Id,
            "center" => Self::Position,
            "scale" => Self::Size,
            "text" => Self::Text,
            "primary" => Self::Primary,
            "secondary" => Self::Secondary,
            _ => panic!("Error: Unknown parameter: '{s}'")
        }
    }
}

/// The list of valid widgets
#[derive(Debug)]
enum Widget {
    Frame,
    Label,
    Button,
    TextInput
}

impl From<&str> for Widget {
    /// Also checks if `s` is a valid widget name
    fn from(s: &str) -> Self {
        match s {
            "Frame" => Self::Frame,
            "Label" => Self::Label,
            "Button" => Self::Button,
            "TextInput" => Self::TextInput,
            _ => panic!("Unknown widget: '{s}'")
        }
    }
}

/// Used in lexing to aggregate any `</` into a single token
#[derive(Copy, Clone)]
enum ParsingState {
    Initial,
    HasOpenened
}

#[derive(PartialEq, Clone, Debug)]
enum Terminal {
    OpeningTag,
    ClosingTag,
    EndingTag,
    Identifier (String),
    Assignation,
    Literal (String),
    Eof,
    Void
}

#[derive(Clone, Debug)]
enum NonTerminal {
    Ui,
    Begin,
    End,
    Params,
    Param,
    Children
}

/// This enum contains every parsing methodes for uilang.
/// Its `ui`, `begin`, `end`, `params`, `param` and `children` methodes
/// represent rules from the context-free grammar listed in the crate's
/// documentation.
/// For short, this is a recursive descent parser.
#[derive(Clone, Debug)]
enum Symbol {
    Terminal (Terminal),
    NonTerminal (NonTerminal)
}

impl Symbol {
    
    fn ui(
        tokens: &mut Vec<Terminal>,
        index: &mut usize,
        widget_stack: &mut Vec<String>,
        generated_code: &mut String
    ) {
        let current = tokens[*index].clone();
        match current {
            Terminal::OpeningTag => {
                Self::begin(tokens, index, widget_stack, generated_code);
                Self::params(tokens, index, widget_stack, generated_code);
                Self::children(tokens, index, widget_stack, generated_code);
                Self::end(tokens, index, widget_stack, generated_code);
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", NonTerminal::Begin)
        }
    }
    
    fn begin(
        tokens: &mut Vec<Terminal>, 
        index: &mut usize, 
        widget_stack: &mut Vec<String>,
        generated_code: &mut String
    ) {
        Self::check_terminal(tokens, Terminal::OpeningTag, index);
        let current = tokens[*index].clone();
        let indentation = "\t".repeat(widget_stack.len() + 1);
        
        match current {
            Terminal::Identifier(value) => {
                widget_stack.push(value.clone());
                let value = Widget::from(&value[..]);
                *index += 1;
                
                println!("Begining a {:?} widget", value);
                
                generated_code.push_str(&format!("Widget::new( WidgetData::{value:?} {{ "));
                generated_code.push_str(
                    match value {
                        Widget::Frame     => "outline: 0.0",
                        Widget::Label     => "text: \"Placeholder\".to_string(), font_size: 60.0",
                        Widget::Button    => "state: ButtonState::Rest",
                        Widget::TextInput => "placeholder: \"Placeholder\".to_string(), input: String::new(), selected: false",
                    }
                );
                generated_code.push_str(
                    &format!(
                        " }} )\n{indentation}.with_relative(Layout {{center: vec2(0.0, 0.0), scale: vec2(1.0, 1.0)}})"
                    )
                );
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", Terminal::OpeningTag)
        }
        Self::check_terminal(tokens, Terminal::ClosingTag, index);
    }
    fn end(
        tokens: &mut Vec<Terminal>, 
        index: &mut usize, 
        widget_stack: &mut Vec<String>,
        generated_code: &mut String
    ) {
        Self::check_terminal(tokens, Terminal::EndingTag, index);
        let current = tokens[*index].clone();
        match current {
            Terminal::Identifier(value) => {
                match widget_stack.pop() {
                    None => panic!("Unmatched ending tag: {value}"),
                    Some(top) => if top != value {
                        panic!("Tried to close a {value} when next in stack is a {top}");
                    }
                } 
                *index += 1;
                println!("Ending a {:?} widget", value);
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", Terminal::OpeningTag)
        }
        Self::check_terminal(tokens, Terminal::ClosingTag, index);
    }
    fn params(
        tokens: &mut Vec<Terminal>, 
        index: &mut usize,
        widget_stack: &mut Vec<String>,
        generated_code: &mut String
    ) {
        let current = tokens[*index].clone();
        match current {
            Terminal::Identifier(_) => {
                Self::param(tokens, index, widget_stack, generated_code);
                Self::params(tokens, index, widget_stack, generated_code);
            },
            Terminal::OpeningTag | Terminal::EndingTag => {
                // Do nothing (produces epsilon)
            },
            _ => panic!("Unexpected token : {current:?}. Expected parameters, children or closing")
        }
    }
    fn param(
        tokens: &mut Vec<Terminal>,
        index: &mut usize,
        widget_stack: &mut Vec<String>,
        generated_code: &mut String
    ) {
        let current = tokens[*index].clone();
        let indentation = "\t".repeat(widget_stack.len());
        
        match current {
            Terminal::Identifier(id) => {
                let parsed_id = Parameter::from(&id[..]);
                *index += 1;
                Self::check_terminal(tokens, Terminal::Assignation, index);
                if let Terminal::Literal(value) = tokens[*index].clone() {
                    *index += 1;
                    println!("Defining a {:?} as ''{:?}'", parsed_id, value);
                    
                    if id != "text" {
                        let (first, last) = (value.chars().next().unwrap(), value.chars().rev().next().unwrap());
                        if (first, last) == ('(', ')') {
                            let s = value
                                .chars()
                                .filter(|x| *x != ')' && *x != '(')
                                .collect::<String>();
                            let s = s.split(",")
                                .into_iter()
                                .collect::<Vec<_>>();
                            
                            if s.len() != 2 { panic!("Expected 2D coordinates, found {}D", s.len()) }
                            
                            generated_code.push_str(&format!("\n{indentation}.with_{id}(vec2({}, {}))", s[0], s[1]));
                        } else if id != "id"{
                            let value: String = value.chars().filter(|x| *x != '"').collect();
                            generated_code.push_str(&format!("\n{indentation}.with_{id}({value}.into())"));
                        } else {
                            generated_code.push_str(&format!("\n{indentation}.with_{id}({value}.into())"));
                        }
                    } else {
                        match widget_stack.last() {
                            Some(s) => {
                                match &s[..] {
                                    "Label" => {
                                        let where_to_change = generated_code.rfind("text: \"Placeholder\".to_string(),");
                                        match where_to_change {
                                            Some(index) =>
                                                generated_code.replace_range(
                                                    (index + 6)..(index + 19),
                                                    &value
                                                ),
                                            None => panic!("This error should never happen ! Found a `text` definition inside a `Label` markup with no placeholder text !")
                                        }
                                        
                                    },
                                    _ => panic!("Found `text` property definition outside of label definition")
                                }
                            },
                            None => panic!("Found `text` property definition outside of widget definition")
                        }
                    }
                } else {
                    panic!("Unexpected {:?}. Expected a literal", tokens[*index]);
                }
                
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", Terminal::Identifier(String::from("any")))
        }
    }
    fn children(
        tokens: &mut Vec<Terminal>, 
        index: &mut usize, 
        widget_stack: &mut Vec<String>,
        generated_code: &mut String
    ) {
        let current = tokens[*index].clone();
        let indentation = "\t".repeat(widget_stack.len());
        
        match current {
            Terminal::OpeningTag => {
                println!("Next line is a child");
                
                generated_code.push_str(&format!("\n{indentation}.with_child("));
                Self::ui(tokens, index, widget_stack, generated_code);
                generated_code.push_str(&format!("\n{indentation})"));
                Self::children(tokens, index, widget_stack, generated_code);
            },
            Terminal::EndingTag => {
                // Do nothing (produces epsilon)
            },
            Terminal::Identifier(id) => {
                panic!("Found parameter `{id}` after child definition. Hint: parameters are only allowed before any child definition")  
            },
            _ => panic!("Unexpected token : {current:?}. Expected ClosingTag or child definition")
        } 
    }
    
    fn check_terminal(source: &mut Vec<Terminal>, would_like: Terminal, index: &mut usize) {
        let to_check = source[*index].clone();
        if to_check == would_like {
            *index += 1;
        } else {
            panic!("Unexpected token: {to_check:?}. Expected {would_like:?}");
        }
    } 
    
}

/// Translates your `uilang` code into correct Rust with [`desi-ui`](https://github.com/sama-gharib/dungeons-together/tree/main/desi-ui/)
#[proc_macro]
pub fn uilang(input: TokenStream) -> TokenStream {
    
    // Lexing
    // Mostly just aggregates '</' into a single token.
    // This code is a bit convoluted for such a simple purpose but
    // it makes it scalable for the eventuality of some more preprocessing
    // needed.
    let mut state = ParsingState::Initial;
    let mut parsed: Vec<Terminal> = Vec::new();
    
    for token in input {
        state = match token {
            TokenTree::Ident(ident) => {
                parsed.push(Terminal::Identifier(ident.to_string()));
                ParsingState::Initial
            },
            TokenTree::Punct(punct) => {
                let c = punct.as_char();
                match c {
                    ':' => {
                        parsed.push(Terminal::Assignation);
                        ParsingState::Initial
                    },
                    '>' => {
                        parsed.push(Terminal::ClosingTag);
                        ParsingState::Initial
                    },
                    '<' => {
                        parsed.push(Terminal::OpeningTag);
                        ParsingState::HasOpenened
                    },
                    '/' => {
                        match state {
                            ParsingState::Initial => panic!("Found '/' not after '<'"),
                            ParsingState::HasOpenened => {
                                let _ = parsed.pop();
                                parsed.push(Terminal::EndingTag);
                                ParsingState::Initial
                            }
                        }
                    }
                    _ => panic!("Uninterpreted character: {c}")
                }
            },
            TokenTree::Literal(literal) => {
                parsed.push(Terminal::Literal(literal.to_string()));
                ParsingState::Initial
            },
            TokenTree::Group(_) => panic!("Invalid syntax: `Group`s are not supported")
        }
    }
    parsed.push(Terminal::Eof);
    
    // Parsing
    
    let mut index = 0;
    let mut widget_stack = Vec::new();
    let mut generated_code = String::new();
    
    Symbol::ui(&mut parsed, &mut index, &mut widget_stack, &mut generated_code);
    
    println!("{generated_code}");
    
    
    generated_code.parse().unwrap()
    // "0u8".parse().unwrap()
}
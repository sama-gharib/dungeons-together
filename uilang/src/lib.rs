use proc_macro::{ TokenStream, TokenTree };

#[derive(Debug)]
enum Parameter {
    Position,
    Size,
    Text,
    Primary,
    Secondary
}

impl From<&str> for Parameter {
    fn from(s: &str) -> Self {
        match s {
            "position" => Self::Position,
            "size" => Self::Size,
            "text" => Self::Text,
            "primary" => Self::Primary,
            "secondary" => Self::Secondary,
            _ => panic!("Unknown parameter: '{s}'")
        }
    }
}

#[derive(Debug)]
enum Widget {
    Frame,
    Label,
    Button
}

impl From<&str> for Widget {
    fn from(s: &str) -> Self {
        match s {
            "Frame" => Self::Frame,
            "Label" => Self::Label,
            "Button" => Self::Button,
            _ => panic!("Unknown widget: '{s}'")
        }
    }
}


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

#[derive(Clone, Debug)]
enum Symbol {
    Terminal (Terminal),
    NonTerminal (NonTerminal)
}

impl Symbol {
    
    fn ui(tokens: &mut Vec<Terminal>, index: &mut usize, widget_stack: &mut Vec<String>) {
        let current = tokens[*index].clone();
        match current {
            Terminal::OpeningTag => {
                Self::begin(tokens, index, widget_stack);
                Self::params(tokens, index);
                Self::children(tokens, index, widget_stack);
                Self::end(tokens, index, widget_stack);
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", NonTerminal::Begin)
        }
    }
    
    fn begin(tokens: &mut Vec<Terminal>, index: &mut usize, widget_stack: &mut Vec<String>) {
        Self::check_terminal(tokens, Terminal::OpeningTag, index);
        let current = tokens[*index].clone();
        match current {
            Terminal::Identifier(value) => {
                widget_stack.push(value.clone());
                let value = Widget::from(&value[..]);
                *index += 1;
                println!("Begining a {:?} widget", value);
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", Terminal::OpeningTag)
        }
        Self::check_terminal(tokens, Terminal::ClosingTag, index);
    }
    fn end(tokens: &mut Vec<Terminal>, index: &mut usize, widget_stack: &mut Vec<String>) {
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
    fn params(tokens: &mut Vec<Terminal>, index: &mut usize) {
        let current = tokens[*index].clone();
        match current {
            Terminal::Identifier(_) => {
                Self::param(tokens, index);
                Self::params(tokens, index);
            },
            Terminal::OpeningTag | Terminal::EndingTag => {
                // Do nothing (produces epsilon)
            },
            _ => panic!("Unexpected token : {current:?}. Expected paramters, children or closing")
        }
    }
    fn param(tokens: &mut Vec<Terminal>, index: &mut usize) {
        let current = tokens[*index].clone();
        match current {
            Terminal::Identifier(id) => {
                let id = Parameter::from(&id[..]);
                *index += 1;
                Self::check_terminal(tokens, Terminal::Assignation, index);
                if let Terminal::Literal(value) = tokens[*index].clone() {
                    *index += 1;
                    println!("Defining a {:?} as ''{:?}'", id, value);
                } else {
                    panic!("Unexpected {:?}. Expected a literal", tokens[*index]);
                }
                
            },
            _ => panic!("Unexpected token : {current:?}. Expected {:?}", Terminal::Identifier(String::from("any")))
        }
    }
    fn children(tokens: &mut Vec<Terminal>, index: &mut usize, widget_stack: &mut Vec<String>) {
        let current = tokens[*index].clone();
        match current {
            Terminal::OpeningTag => {
                println!("Next line is a child");
                Self::ui(tokens, index, widget_stack);
                Self::children(tokens, index, widget_stack);
            },
            Terminal::EndingTag => {
                // Do nothing (produces epsilon)
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

#[proc_macro]
pub fn uilang(input: TokenStream) -> TokenStream {
    
    // Lexing
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
    Symbol::ui(&mut parsed, &mut index, &mut widget_stack);
    
    TokenStream::new()
}
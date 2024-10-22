use logos::{Lexer, Logos};

mod meta;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokensj
enum Token {
    #[token("brew")]
    Brew,

    #[token("cask")]
    Cask,

    #[token("args:")]
    Args,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,


    #[token(",")]
    Comma,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    String(String),

    #[regex(r#"#{1}"#)]
    Comment,

    #[token("\n")]
    NewLine,
}

#[derive(Debug)]
enum Command {
    Brew(String, Vec<BrewCommand>),
}

#[derive(Debug)]
enum BrewCommand {
    Args(Vec<String>),
}

fn parse_list(lexer: &mut Lexer<'_, Token>) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    let mut awaits_comma = false;
    let mut awaits_value = false;

    for token in lexer.by_ref() {
        match token {
            // Open brace. We expect a value next
            Ok(Token::OpenBracket) if !awaits_comma => awaits_value = true,
            // Value received. We expect a comma next if more than one.
            Ok(Token::String(text)) if !awaits_comma => {
                list.push(text);
                awaits_value = false;
            },
            // Comma received. We expect a value next.
            Ok(Token::Comma) if awaits_comma => awaits_value = true,
            // Close brace if no value is waiting. Break
            Ok(Token::CloseBracket) if !awaits_value => break,
            // Any other value is invalid.
            _ => panic!("Invalid token in list."),
        };
        awaits_comma = !awaits_value;
    }
    list
}

fn parse_brew(lexer: &mut Lexer<'_, Token>) ->  Command {
    let mut target: String = String::new();
    let mut list: Vec<BrewCommand> = Vec::new();
    let mut awaits_comma = false;
    let mut awaits_value = false;

    // for token in lexer.by_ref() {
    while let Some(token) = lexer.next() {
        match token {
            // Target is always the first parameter.
            Ok(Token::String(text)) if target.is_empty() => target = text,
            // Comma received. Expect parameters next
            Ok(Token::Comma) if awaits_comma => awaits_value = true,
            // Argument received. Expect comma next
            Ok(Token::Args) if !awaits_comma => {
                let parsed_list = parse_list(lexer);
                list.push(BrewCommand::Args(parsed_list));
                awaits_value = false;
            },
            // Comment received. End of command.
            Ok(Token::Comment) if awaits_comma => break,
            // Breakline received. End of command.
            Ok(Token::NewLine) if awaits_comma => break,

            _ => panic!("Invalid brew command"),
        };
        awaits_comma = !awaits_value;
    }
    Command::Brew(target, list)
}

fn parse_lexer(text: &str) {
    let mut lex = Token::lexer(text);

    while let Some(token) = lex.next() {
        match token {
            Ok(Token::Brew) => {
                let brew = parse_brew(&mut lex);
                println!("--- {:#?}", brew);
            },
            Ok(_) => (),
            Err(error) => {
                println!("----- error: {:#?}", error);
                continue;
            }
        };
    }
}

fn main() {
    let text = String::from(r#"
        ## Other text that doesn't have stuff
        ## @required
        ## @description hello world
        ## Other text that doesn't have mooooore stuff
        brew "asdf", args: ["hello", "world"]
        brew "node" #, args: ["world"]
        brew "neovim" #, args: ["foo"]
     "#);

    // parse_lexer(text.as_str());
    meta::parse_meta(text.as_str());
    println!("End of file");
}


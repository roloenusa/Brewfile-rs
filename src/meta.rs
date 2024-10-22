use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Clone)]
enum MetaToken {
    // #[token("@description", get_content)]
    // Doc(String),

    // #[token("@required")]
    // Required,

    #[regex(r"## @description[^\n]", get_content)]
    Text(String),

    #[token(r"## @required")]
    Required,

    #[regex(r"## [^\n]*", get_content)]
    OtherText(String),

    #[token("##")]
    Comment,

    #[token("\n")]
    NewLine,
}

fn get_content(lex: &mut Lexer<MetaToken>) -> String {
    let binding = lex.slice().replace("##", "").replace("@description", "");
    let slice = binding.trim();
    String::from(slice)
}

pub fn parse_meta(text: &str) {
    let mut lex = MetaToken::lexer(text);

    while let Some(token) = lex.next() {
        // println!("--- {:#?}", token);
        match token {
            // Ok(MetaToken::Doc(text)) => {
            //     println!("Doc: {}", text)
            // },
            Ok(MetaToken::OtherText(line)) => {
                println!("Other: {}", line)
            },
            Ok(MetaToken::Text(line)) => {
                println!("Line: {}", line)
            },
            Ok(MetaToken::Required) => {
                println!("required: true")
            },
            _ => (),
        };
    }
}


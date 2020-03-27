use mathew::Eval;
use std::process::exit;
use synterm::{gen_lexer, gen_parse, Color, CommandLineTool};

struct MyTool;

impl CommandLineTool for MyTool {
    fn evaluator_function(line: &String) -> String {
        match line.as_str() {
            "exit" => {
                exit(0);
            }
            _ => {
                let e = Eval::default();
                match e.eval(line) {
                    Some(n) => format!("{}", n),
                    None => "Error".to_string(),
                }
            }
        }
    }
    fn syntax_highlight(string: &str) {
        gen_lexer!(
            TheLexer,
            (KeyWord, "exit"),
            (Number, "[0-9]+"),
            (Paren, r"[\(\)]"),
            (Operator, r"[\+-/\*%]")
        );
        gen_parse!(
            TheLexer,
            parser,
            (Number, Color::Blue),
            (Paren, Color::Magenta),
            (Operator, Color::Yellow),
            (KeyWord, Color::Red)
        );
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    MyTool.start();
}

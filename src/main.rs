use std::process::exit;
use synterm::{CommandLineTool, gen_lexer, gen_parse};
use mathew::Eval;

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
                    Some(n) => {
                        format!("{}", n)
                    }
                    None => {
                        "Error".to_string()
                    }
                }
            }
        }
    }
    fn syntax_highlight(string: &str) {
        gen_lexer!(TheLexer, (Number, "[0-9]+"), (Paren, r"[\(\)]"), (Operator, r"[\+-/\*]"));
        gen_parse!(TheLexer, parser, (Number,"34"), (Paren,"32"), (Operator, "33"));
        parser(TheLexer::lexer(string));
    }
}

fn main() {
    let command_line = MyTool;
    command_line.start();
}

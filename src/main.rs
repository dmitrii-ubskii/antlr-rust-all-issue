use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;

mod all_testlexer;
mod all_testlistener;
mod all_testparser;
mod all_testvisitor;

use all_testlexer::all_testLexer;
use all_testparser::*;
use all_testvisitor::all_testVisitorCompat;

struct Parser;

impl Parser {
    fn parse(&mut self, query: &str) -> String {
        let lexer = all_testLexer::new(InputStream::new(query));
        let mut parser = all_testParser::new(CommonTokenStream::new(lexer));
        self.visit_list(parser.list().unwrap().as_ref())
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for Parser {
    type Node = all_testParserContextType;
    type Return = String;

    fn temp_result(&mut self) -> &mut Self::Return {
        panic!("temp_result")
    }

    fn aggregate_results(&self, _aggregate: Self::Return, _next: Self::Return) -> Self::Return {
        panic!("aggregate_results")
    }
}

impl<'input> all_testVisitorCompat<'input> for Parser {
    // fn visit_list(&mut self, ctx: &ListContext<'input>) -> Self::Return {
    //     ctx.VAR__all().into_iter().map(|x| x.symbol.get_text().to_string()).collect::<Vec<String>>().join(" ")
    // }

    fn visit_list(&mut self, ctx: &ListContext<'input>) -> Self::Return {
        let mut i = 0;
        let mut buffer = String::new();
        loop {
            match ctx.VAR_(i) {
                Some(var) => {
                    if buffer.len() > 0 {
                        buffer.push(' ');
                    }
                    buffer.push_str(var.symbol.get_text());
                },
                None => break,
            }
            i += 1;
        }
        buffer
    }
}

fn main() {
    println!("{}", Parser {}.parse("get $abc, $def"))
}

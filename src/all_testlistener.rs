#![allow(nonstandard_style)]
// Generated from all_test.g4 by ANTLR 4.8
use super::all_testparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait all_testListener<'input>: ParseTreeListener<'input, all_testParserContextType> {
    /**
     * Enter a parse tree produced by {@link all_testParser#list}.
     * @param ctx the parse tree
     */
    fn enter_list(&mut self, _ctx: &ListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link all_testParser#list}.
     * @param ctx the parse tree
     */
    fn exit_list(&mut self, _ctx: &ListContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : all_testListener<'input> }

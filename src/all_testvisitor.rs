#![allow(nonstandard_style)]
// Generated from all_test.g4 by ANTLR 4.8
use super::all_testparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link all_testParser}.
 */
pub trait all_testVisitor<'input>: ParseTreeVisitor<'input, all_testParserContextType> {
    /**
     * Visit a parse tree produced by {@link all_testParser#list}.
     * @param ctx the parse tree
     */
    fn visit_list(&mut self, ctx: &ListContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait all_testVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = all_testParserContextType>
{
    /**
     * Visit a parse tree produced by {@link all_testParser#list}.
     * @param ctx the parse tree
     */
    fn visit_list(&mut self, ctx: &ListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> all_testVisitor<'input> for T
where
    T: all_testVisitorCompat<'input>,
{
    fn visit_list(&mut self, ctx: &ListContext<'input>) {
        let result = <Self as all_testVisitorCompat>::visit_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}

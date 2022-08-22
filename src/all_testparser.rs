// Generated from all_test.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::all_testlistener::*;
use super::all_testvisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const GET: isize = 2;
pub const VAR_: isize = 3;
pub const WS: isize = 4;
pub const RULE_list: usize = 0;
pub const ruleNames: [&'static str; 1] = ["list"];

pub const _LITERAL_NAMES: [Option<&'static str>; 3] = [None, Some("','"), Some("'get'")];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 5] =
    [None, None, Some("GET"), Some("VAR_"), Some("WS")];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    all_testParserExt<'input>,
    I,
    all_testParserContextType,
    dyn all_testListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type all_testTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, all_testParserContextType, dyn all_testListener<'input> + 'a>;

/// Parser for all_test grammar
pub struct all_testParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> all_testParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                all_testParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> all_testParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> all_testParser<'input, I, DefaultErrorStrategy<'input, all_testParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for all_testParser
pub trait all_testParserContext<'input>:
    for<'x> Listenable<dyn all_testListener<'input> + 'x>
    + for<'x> Visitable<dyn all_testVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = all_testParserContextType>
{
}

antlr_rust::coerce_from! { 'input : all_testParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn all_testParserContext<'input> + 'input
where
    T: all_testVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn all_testVisitor<'input> + 'x))
    }
}

impl<'input> all_testParserContext<'input> for TerminalNode<'input, all_testParserContextType> {}
impl<'input> all_testParserContext<'input> for ErrorNode<'input, all_testParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn all_testParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn all_testListener<'input> + 'input }

pub struct all_testParserContextType;
antlr_rust::tid! {all_testParserContextType}

impl<'input> ParserNodeType<'input> for all_testParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn all_testParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for all_testParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for all_testParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct all_testParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> all_testParserExt<'input> {}
antlr_rust::tid! { all_testParserExt<'a> }

impl<'input> TokenAware<'input> for all_testParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for all_testParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for all_testParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "all_test.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- list ----------------
pub type ListContextAll<'input> = ListContext<'input>;

pub type ListContext<'input> = BaseParserRuleContext<'input, ListContextExt<'input>>;

#[derive(Clone)]
pub struct ListContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> all_testParserContext<'input> for ListContext<'input> {}

impl<'input, 'a> Listenable<dyn all_testListener<'input> + 'a> for ListContext<'input> {
    fn enter(&self, listener: &mut (dyn all_testListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_list(self);
    }
    fn exit(&self, listener: &mut (dyn all_testListener<'input> + 'a)) {
        listener.exit_list(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn all_testVisitor<'input> + 'a> for ListContext<'input> {
    fn accept(&self, visitor: &mut (dyn all_testVisitor<'input> + 'a)) {
        visitor.visit_list(self);
    }
}

impl<'input> CustomRuleContext<'input> for ListContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = all_testParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_list
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_list }
}
antlr_rust::tid! {ListContextExt<'a>}

impl<'input> ListContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn all_testParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ListContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ListContextExt { ph: PhantomData },
        ))
    }
}

pub trait ListContextAttrs<'input>:
    all_testParserContext<'input> + BorrowMut<ListContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token GET
    /// Returns `None` if there is no child corresponding to token GET
    fn GET(&self) -> Option<Rc<TerminalNode<'input, all_testParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GET, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token VAR_ in current rule
    fn VAR__all(&self) -> Vec<Rc<TerminalNode<'input, all_testParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token VAR_, starting from 0.
    /// Returns `None` if number of children corresponding to token VAR_ is less or equal than `i`.
    fn VAR_(&self, i: usize) -> Option<Rc<TerminalNode<'input, all_testParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VAR_, i)
    }
}

impl<'input> ListContextAttrs<'input> for ListContext<'input> {}

impl<'input, I, H> all_testParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn list(&mut self) -> Result<Rc<ListContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_list);
        let mut _localctx: Rc<ListContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(2);
                recog.base.match_token(GET, &mut recog.err_handler)?;

                recog.base.set_state(3);
                recog.base.match_token(VAR_, &mut recog.err_handler)?;

                recog.base.set_state(8);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__0 {
                    {
                        {
                            recog.base.set_state(4);
                            recog.base.match_token(T__0, &mut recog.err_handler)?;

                            recog.base.set_state(5);
                            recog.base.match_token(VAR_, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(10);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x06\x0e\x04\x02\x09\x02\x03\x02\x03\x02\x03\x02\x03\x02\x07\x02\x09\x0a\
	\x02\x0c\x02\x0e\x02\x0c\x0b\x02\x03\x02\x02\x02\x03\x02\x02\x02\x02\x0d\
	\x02\x04\x03\x02\x02\x02\x04\x05\x07\x04\x02\x02\x05\x0a\x07\x05\x02\x02\
	\x06\x07\x07\x03\x02\x02\x07\x09\x07\x05\x02\x02\x08\x06\x03\x02\x02\x02\
	\x09\x0c\x03\x02\x02\x02\x0a\x08\x03\x02\x02\x02\x0a\x0b\x03\x02\x02\x02\
	\x0b\x03\x03\x02\x02\x02\x0c\x0a\x03\x02\x02\x02\x03\x0a";

// main.rs
//
// This application will read a .pglsf file and output a .rs file
// that compiles into a lib for parsing the specified language.
#![allow(non_camel_case_types)]

mod parse_machine;
mod list;

//use parse_machine::ParseMachine;
use parse_machine::ParseRule;
use parse_machine::SymbolOrRule;
use list::*;

#[derive(Copy, Clone)]
enum Symbol {
    ULETTER_A,
    ULETTER_B,
    ULETTER_C,
    ULETTER_D,
    ULETTER_E,
    ULETTER_F,
    ULETTER_G,
    ULETTER_H,
    ULETTER_I,
    ULETTER_J,
    ULETTER_K,
    ULETTER_L,
    ULETTER_M,
    ULETTER_N,
    ULETTER_O,
    ULETTER_P,
    ULETTER_Q,
    ULETTER_R,
    ULETTER_S,
    ULETTER_T,
    ULETTER_U,
    ULETTER_V,
    ULETTER_W,
    ULETTER_X,
    ULETTER_Y,
    ULETTER_Z,

    LETTER_A,
    LETTER_B,
    LETTER_C,
    LETTER_D,
    LETTER_E,
    LETTER_F,
    LETTER_G,
    LETTER_H,
    LETTER_I,
    LETTER_J,
    LETTER_K,
    LETTER_L,
    LETTER_M,
    LETTER_N,
    LETTER_O,
    LETTER_P,
    LETTER_Q,
    LETTER_R,
    LETTER_S,
    LETTER_T,
    LETTER_U,
    LETTER_V,
    LETTER_W,
    LETTER_X,
    LETTER_Y,
    LETTER_Z,

    DIGIT_0,
    DIGIT_1,
    DIGIT_2,
    DIGIT_3,
    DIGIT_4,
    DIGIT_5,
    DIGIT_6,
    DIGIT_7,
    DIGIT_8,
    DIGIT_9,

    UNDERSCORE,
    COMMA,
    SEMICOLON,
    LEFT_PAREN,
    RIGHT_PAREN,
    EQUAL_SIGN,
    QUESTION_MARK,
    ASTERISK,
    PLUS_SIGN,
    PIPE,
    POUND_SIGN,

    SPACE,
    TAB,
    NEWLINE,

    PERIOD,
    SLASH,
    DASH,
    COLON,
}

#[derive(Copy, Clone)]
enum Rule {
    ROOT,

    // GENERIC
    LETTER,
    LOWER_LETTER,
    UPPER_LETTER,
    DIGIT,
    ELLIPSIS,

    WHITESPACE,
    LINE_END,
    WS_OR_LE,

    COMMA_SEP,
    SEMICOLON_SEP,

    LOWER_NAME,
    UPPER_NAME,

    // COMMENT
    COMMENT,
    COMMENT_TEXT,
    COMMENT_PUNCTUATION,

    // FILE
    FILE,
    THE_WORD_SYMBOLS,
    THE_WORD_GRAMMAR,

    // SYMBOL LIST
    SYMBOL_LIST,
    SYMBOL_NAME,

    THE_WORD_BINARY,

    // RULE LIST
    RULE_LIST,
    RULE,
    RULE_NAME,

    RULE_EXPR,

    RULE_SEQ_EXPR,

    RULE_SUBST_EXPR,
    RULE_SYMBOL_SUBST_EXPR,
    RULE_RULE_SUBST_EXPR,

    RULE_PAREN_EXPR,

    RULE_OPT_EXPR,
    RULE_STAR_EXPR,
    RULE_PLUS_EXPR,

    RULE_UNION_EXPR,
    RULE_UNION_EXPR_ARG,
    UNION_OPERATOR,

    RULE_RANGE_EXPR,
    RANGE_OPERATOR,
}

impl ParseRule<Symbol, Rule> for Rule {
    fn root() -> Rule { Rule::ROOT }

    fn execute(&self, stack: List<SymbolOrRule<Symbol, Rule>>) -> Vec<List<SymbolOrRule<Symbol, Rule>>> {
        let mut result: Vec<List<SymbolOrRule<Symbol, Rule>>> = Vec::new();

        //match (self, stack.state()) {
            //(ROOT, NonEmptyList(head, tail)) => (),

        //}
        //match (self, first) {
        //    (ROOT, Symbol:: => {
        //        result.push(stack);
        //    }
        //    _ => panic!()
        //};

        result
    }
}

fn main() {

}

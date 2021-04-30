use crate::ast::{Ast, Range, TypeAst};
use peg;

const KEYWORDS: &[&str] = &[
    "in",
    "let",
    "int",
    "bool",
    "and",
    "true",
    "false",
];

peg::parser! {
    grammar grammar() for str {
        rule _ = quiet! { (" " / "\t" / "\n" / "\r")* { } }

        rule u32() -> u32
            = quiet! { n:$(['0'..='9']+) {?
            n.parse().or(Err("u32"))
        } }
        / expected!("number")

        rule i32() -> i32 = quiet! { sign:$(("-" / "+")*) u:u32() {
            let mut ret = u as i32;
            for c in sign.chars() {
                if c == '-' {
                    ret *= -1;
                }
            }
            return ret;
        } }
        / expected!("signed number")

        rule ident_start_char()
            = ['a'..='z' | 'A'..='Z' | '_' | '?' | '!']

        rule ident_char()
            = ident_start_char() / ['0'..='9']

        rule ident() -> String
            = quiet!{ s:$(ident_start_char() ident_char()*) {?
                if KEYWORDS.iter().all(|x| *x != s) {
                    Ok(s.to_string())
                } else { Err("Unexpected keyword") }
            } }
            / expected!("identifier")

        pub rule type_atom() -> TypeAst
            = begin:position!() "(" _ t:type_expr() _ ")" end:position!() { TypeAst::Paren(Box::new(t), Range(begin, end)) }
            / begin:position!() "int" end:position!() { TypeAst::Int(Range(begin, end)) }
            / begin:position!() "bool" end:position!() { TypeAst::Bool(Range(begin, end)) }

        pub rule type_expr() -> TypeAst = precedence! {
            l:@ _ "->" _ r:(@) { TypeAst::Function(Box::new(l), Box::new(r)) }
            --
            e:type_atom() { e }
        }

        pub rule int_literal() -> Ast
            = begin:position!() n:i32() end:position!()
            { Ast::IntLiteral(n, Range(begin, end)) }

        pub rule bool_literal() -> Ast
            = begin:position!() b:("true" {true} / "false" {false}) end:position!()
            { Ast::BoolLiteral(b, Range(begin, end)) }

        pub rule atom() -> Ast
            = int_literal()
            / bool_literal()
            / begin:position!() "(" _ e:expr() _ ")" end:position!()
            { Ast::Paren(Box::new(e), Range(begin, end)) }
            / begin:position!() i:ident() end:position!()
            { Ast::Var(i, Range(begin, end)) }

        pub rule arith() -> Ast = precedence! {
            x:(@) _ ">" _ y:@ { Ast::RApp(Box::new(x), Box::new(y)) }
            x:(@) _ "<" _ y:@ { Ast::LApp(Box::new(x), Box::new(y)) }
            --
            x:(@) _ "and" _ y:@ { Ast::And(Box::new(x), Box::new(y)) }
            --
            x:(@) _ "="   _ y:@ { Ast::Eq(Box::new(x), Box::new(y)) }
            --
            x:(@) _ "+"   _ y:@ { Ast::Add(Box::new(x), Box::new(y)) }
            --
            e: atom() { e }
        }

        pub rule let_declaration() -> Ast
            = begin:position!()
            "let" _ name:ident() _ "=" _ right:expr() _ "in" _ body:expr()
            end:position!() {
                Ast::Let {
                    name,
                    right: Box::new(right),
                    body: Box::new(body),
                    range: Range(begin, end),
                }
            }

        pub rule function() -> Ast
            = p1:position!() input:ident() p2:position!() _
            ":" _ input_type:type_atom() _ "->" _ ret:expr()
            { Ast::Function { input: (input, Range(p1, p2)), input_type, ret: Box::new(ret) } }

        pub rule expr() -> Ast
            = let_declaration()
            / function()
            / arith()

        pub rule program() -> Ast
            = _ e:expr() _ { e }
    }
}

pub use grammar::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_literal_test() {
        assert_eq!(
            grammar::int_literal("123"),
            Ok(Ast::IntLiteral(123, Range(0, 3)))
        );
    }

    #[test]
    fn complex_expression() {
        let a = grammar::expr("1 + 2 = 3 and 5 = -3 + 1 + 3 and true");
        let b = Ast::And(
            Box::new(Ast::And(
                Box::new(Ast::Eq(
                    Box::new(Ast::Add(
                        Box::new(Ast::IntLiteral(1, Range(0, 1))),
                        Box::new(Ast::IntLiteral(2, Range(4, 5))),
                    )),
                    Box::new(Ast::IntLiteral(3, Range(8, 9))),
                )),
                Box::new(Ast::Eq(
                    Box::new(Ast::IntLiteral(5, Range(14, 15))),
                    Box::new(Ast::Add(
                        Box::new(Ast::Add(
                            Box::new(Ast::IntLiteral(-3, Range(18, 20))),
                            Box::new(Ast::IntLiteral(1, Range(23, 24))),
                        )),
                        Box::new(Ast::IntLiteral(3, Range(27, 28))),
                    )),
                )),
            )),
            Box::new(Ast::BoolLiteral(true, Range(33, 37))),
        );
        assert_eq!(a, Ok(b));
    }
}

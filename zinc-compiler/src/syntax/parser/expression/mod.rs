//!
//! The expression parser.
//!

pub mod access;
pub mod add_sub;
pub mod and;
pub mod assignment;
pub mod bitwise_and;
pub mod bitwise_or;
pub mod bitwise_shift;
pub mod bitwise_xor;
pub mod casting;
pub mod comparison;
pub mod mul_div_rem;
pub mod or;
pub mod path;
pub mod range;
pub mod terminal;
pub mod xor;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::expression::assignment::Parser as AssignmentOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, Copy)]
pub enum State {
    AssignmentFirstOperand,
    AssignmentOperator,
    AssignmentSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        State::AssignmentFirstOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: ExpressionTreeBuilder,
}

impl Parser {
    ///
    /// Parses a top-level expression.
    ///
    /// 'a = 42'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        loop {
            match self.state {
                State::AssignmentFirstOperand => {
                    let (expression, next) =
                        AssignmentOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::AssignmentOperator;
                }
                State::AssignmentOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Assignment, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::PlusEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentAddition, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::MinusEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentSubtraction, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::AsteriskEquals),
                            location,
                        } => {
                            self.builder.eat_operator(
                                ExpressionOperator::AssignmentMultiplication,
                                location,
                            );
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::SlashEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentDivision, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::PercentEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentRemainder, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::VerticalBarEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentBitwiseOr, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::CircumflexEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentBitwiseXor, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::AmpersandEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::AssignmentBitwiseAnd, location);
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleLesserEquals),
                            location,
                        } => {
                            self.builder.eat_operator(
                                ExpressionOperator::AssignmentBitwiseShiftLeft,
                                location,
                            );
                            self.state = State::AssignmentSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleGreaterEquals),
                            location,
                        } => {
                            self.builder.eat_operator(
                                ExpressionOperator::AssignmentBitwiseShiftRight,
                                location,
                            );
                            self.state = State::AssignmentSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::AssignmentSecondOperand => {
                    let (expression, token) =
                        AssignmentOperandParser::default().parse(stream, None)?;
                    self.builder.eat(expression);
                    return Ok((self.builder.finish(), token));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_assignment() {
        let input = r#"a = 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::Assignment),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 5),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 5),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_bitwise_or() {
        let input = r#"a |= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentBitwiseOr),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_bitwise_xor() {
        let input = r#"a ^= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentBitwiseXor),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_bitwise_and() {
        let input = r#"a &= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentBitwiseAnd),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_bitwise_shift_left() {
        let input = r#"a <<= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentBitwiseShiftLeft),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 7),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_bitwise_shift_right() {
        let input = r#"a >>= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentBitwiseShiftRight),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 7),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_addition() {
        let input = r#"a += 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentAddition),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_subtraction() {
        let input = r#"a -= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentSubtraction),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_multiplication() {
        let input = r#"a *= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentMultiplication),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_division() {
        let input = r#"a /= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentDivision),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_assignment_remainder() {
        let input = r#"a %= 42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 3),
                ExpressionTreeNode::operator(ExpressionOperator::AssignmentRemainder),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "a".to_owned(),
                    ))),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 6),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}

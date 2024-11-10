use std::{iter::Peekable, str::CharIndices};

use token::*;

mod token;

use lalrpop_util::lalrpop_mod;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Space,
    Tab,
    CarriageReturn,
    LineFeed,
    Asterisk,
    Colon,
    At,
    Equal,
    Plus,
    // Minus,
    // Mul,
    // Div,
    // Mod,
    Cacco,
    Cocca,
    Number(f32),
    Identifier(String),
}

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(inputs: &'input str) -> Self {
        Lexer {
            chars: inputs.char_indices().peekable(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        fn tik(
            s: usize,
            token: Token,
            e: usize,
        ) -> Option<Result<(usize, Token, usize), LexicalError>> {
            Some(Ok((s, token, e)))
        }
        match self.chars.next() {
            Some((i, ' ')) => tik(i, Token::Space, i + ' '.len_utf8()),
            Some((i, '\t')) => tik(i, Token::Tab, i + '\t'.len_utf8()),
            Some((i, '\r')) => tik(i, Token::CarriageReturn, i + '\r'.len_utf8()),
            Some((i, '\n')) => tik(i, Token::LineFeed, i + '\n'.len_utf8()),
            Some((i, '＊')) => tik(i, Token::Asterisk, i + '＊'.len_utf8()),
            Some((i, '＠')) => tik(i, Token::At, i + '＠'.len_utf8()),
            Some((i, '：')) => tik(i, Token::Colon, i + '：'.len_utf8()),
            Some((i, '＝')) => match self.chars.next() {
                Some((_, '＝')) => tik(i, Token::Equal, i + '＝'.len_utf8() * 2),
                Some((_, c)) => Some(Err(LexicalError::UnexpectedCharacter(
                    i,
                    c,
                    i + '＝'.len_utf8() * 2,
                ))),
                None => Some(Err(LexicalError::UnexpectedCharacter(
                    i,
                    '＝',
                    i + '＝'.len_utf8() * 2,
                ))),
            },
            Some((i, '＋')) => tik(i, Token::Plus, i + '＋'.len_utf8()),
            // Some((i, '－')) => tik(i, Token::Minus, i + '－'.len_utf8()),
            // Some((i, '×')) => tik(i, Token::Mul, i + '×'.len_utf8()),
            // Some((i, '／')) => tik(i, Token::Div, i + '／'.len_utf8()),
            // Some((i, '％')) => tik(i, Token::Mod, i + '％'.len_utf8()),
            Some((i, '（')) => tik(i, Token::Cacco, i + '（'.len_utf8()),
            Some((i, '）')) => tik(i, Token::Cocca, i + '）'.len_utf8()),
            // Some((s, c)) => {
            //     if c.is_numeric() || c == '.' {
            //         let mut num = c.to_string();
            //         let mut e = s + c.len_utf8();
            //         let mut has_dot = c == '.';
            //         while let Some((_, c)) = self.chars.peek() {
            //             if !c.is_numeric() && *c != '.' {
            //                 return tik(s, Token::Number(num.parse().unwrap()), e);
            //             }

            //             if *c == '.' {
            //                 if has_dot {
            //                     return Some(Err(LexicalError::UnexpectedCharacter(s, *c, s + 1)));
            //                 }
            //                 has_dot = true;
            //             }

            //             e += c.len_utf8();
            //             num.push(*c);
            //             self.chars.next();
            //         }
            //         tik(s, Token::Number(num.parse().unwrap()), e)
            //     } else {
            //         Some(Err(LexicalError::UnexpectedCharacter(s, c, s + 1)))
            //     }
            // }
            Some((s, c)) => {
                if c.is_numeric() || c == '.' {
                    let mut num = c.to_string();
                    let mut e = s + c.len_utf8();
                    let mut has_dot = c == '.';
                    while let Some((_, c)) = self.chars.peek() {
                        if !c.is_numeric() && *c != '.' {
                            return tik(s, Token::Number(num.parse().unwrap()), e);
                        }

                        if *c == '.' {
                            if has_dot {
                                return Some(Err(LexicalError::UnexpectedCharacter(s, *c, s + 1)));
                            }
                            has_dot = true;
                        }

                        e += c.len_utf8();
                        num.push(*c);
                        self.chars.next();
                    }
                    tik(s, Token::Number(num.parse().unwrap()), e)
                } else if c.is_alphabetic() {
                    let mut iden = c.to_string();
                    let mut e = s + c.len_utf8();
                    while let Some((_, c)) = self.chars.peek() {
                        if !c.is_alphabetic() {
                            return tik(s, Token::Identifier(iden), e);
                        }
                        e += c.len_utf8();
                        iden.push(self.chars.next().unwrap().1)
                    }
                    tik(s, Token::Identifier(iden), e)
                } else {
                    Some(Err(LexicalError::UnexpectedCharacter(s, c, s + 1)))
                }
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum LexicalError {
    UnexpectedCharacter(usize, char, usize),
}

pub mod ast {
    #[derive(Debug)]
    pub struct Satori {
        pub talk: Vec<Talk>,
        pub word_group: Vec<WordGroup>,
    }

    #[derive(Debug)]
    pub struct Talk {
        pub start: TalkStart,
        pub contents: Vec<Expression>,
    }

    #[derive(Debug)]
    pub struct TalkStart {
        pub label: Option<Expression>,
        pub condition: Option<Expression>,
    }

    #[derive(Debug)]
    pub struct WordGroup {
        pub label: Expression,
        pub contents: Vec<Expression>,
    }

    #[derive(Debug)]
    pub enum Expression {
        Binary(BinaryExpression),
        Term(Term),
    }

    #[derive(Debug)]
    pub struct BinaryExpression {
        pub lhs: Box<Expression>,
        pub op: Op,
        pub rhs: Term,
    }

    #[derive(Debug)]
    pub enum Term {
        Binary(BinaryTerm),
        Factor(Factor),
    }

    #[derive(Debug)]
    pub struct BinaryTerm {
        pub lhs: Box<Term>,
        pub op: Op,
        pub rhs: Factor,
    }

    #[derive(Debug)]
    pub enum Factor {
        Expression(Box<Expression>),
        String(String),
        Number(f32),
    }

    #[derive(Debug)]
    pub enum Op {
        Plus,
        Minus,
        Mul,
        Div,
        Mod,
        Equal,
    }
}

lalrpop_mod!(pub satori); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculator1() {
        for result in Lexer::new("＊test\tleft＝＝right\nこんにちは\r\n：おはろー") {
            result
                .map(|(s, t, e)| println!("{:?} {} {}", t, s, e))
                .map_err(|e| println!("{:?}", e))
                .unwrap();
        }
        let lexer = Lexer::new("＊test\tleft＝＝right\nこんにちは\r\n：おはろー");
        let parser = satori::SatoriParser::new();
        assert!(parser
            .parse(lexer)
            .map(|r| println!("success {:?}", r))
            .map_err(|e| println!("failed {:?}", e))
            .is_ok());

        for result in Lexer::new("＠てすと\nあさ\nひる\nよる") {
            result
                .map(|(s, t, e)| println!("{:?} {} {}", t, s, e))
                .map_err(|e| println!("{:?}", e))
                .unwrap();
        }
        assert!(parser
            .parse(Lexer::new("＠てすと\nあさ\nひる\nよる"))
            .map(|r| println!("success word group {:?}", r))
            .is_ok());

        assert_eq!(true, false);
    }

    #[test]
    pub fn it_works() {
        let result = parse(
            vec![
                "＊",
                "OnBoot",
                "：",
                "（",
                "iflist",
                "、",
                "（",
                "現在時",
                "）",
                "、",
                "＜",
                "６",
                "、",
                "こんばんは。",
                "、",
                "＜",
                "１１",
                "、",
                "おはようございます。",
                "、",
                "＜",
                "１８",
                "、",
                "こんにちは。",
                "、",
                "＜",
                "２４",
                "、",
                "こんばんは。",
                "）",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
        );

        assert_eq!(result.is_ok(), true)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Asta(String),
    Colon(String),
    At(String),
    Dollar(String),
    CaccoOpen(String),
    CaccoClose(String),
    Sharp(String),
    Greater(String),
    GreaterGreater(String),
    GreaterEqual(String),
    Underbar(String),
    Arrow(String),
    Sentense(String),
}

impl Token {
    pub fn if_not_first_token_then_none(self) -> Option<Self> {
        match self {
            Token::Asta(_) => Some(self),
            Token::Colon(_) => None,
            Token::At(_) => None,
            Token::Dollar(_) => None,
            Token::CaccoOpen(_) => None,
            Token::CaccoClose(_) => None,
            Token::Sharp(_) => None,
            Token::Greater(_) => None,
            Token::GreaterGreater(_) => None,
            Token::GreaterEqual(_) => None,
            Token::Underbar(_) => None,
            Token::Arrow(_) => None,
            Token::Sentense(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn if_not_first_token_then_none() {
        assert_eq!(
            Token::Asta("＊".to_string()).if_not_first_token_then_none(),
            Some(Token::Asta("＊".to_string()))
        );
        assert_eq!(
            Token::Colon("：".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::At("＠".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::Dollar("＄".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::CaccoOpen("（".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::CaccoClose("）".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::Sharp("＃".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::Greater("＞".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::GreaterGreater("≫".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::GreaterEqual("≧".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::Underbar("＿".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::Arrow("→".to_string()).if_not_first_token_then_none(),
            None
        );
        assert_eq!(
            Token::Sentense("OnBoot".to_string()).if_not_first_token_then_none(),
            None
        );
    }
}

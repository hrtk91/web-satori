use node::*;
use token::*;

mod node;
mod token;

pub fn analyze(mut tokens: Vec<String>) {
    // 最初の1つめを取り出して1つめとして許可される文字列じゃなかったらエラー
    let first = tokens.drain(0..1).next().unwrap();
    let mut node: Node = Node::new(
        sorting_hat(first)
            .if_not_first_token_then_none()
            .expect("最初のトークンが不正です"),
    );

    for token in tokens {
        let token = sorting_hat(token);
        node.addChild(Node::new(token));
    }

    // let tokens: Vec<Token> = tokens.into_iter().map(|token| sorting_hat(token)).collect();
}

fn sorting_hat(str: String) -> Token {
    match str.as_str() {
        "＊" => Token::Asta(str),
        "：" => Token::Colon(str),
        "＠" => Token::At(str),
        "＄" => Token::Dollar(str),
        "（" => Token::CaccoOpen(str),
        "）" => Token::CaccoClose(str),
        "＃" => Token::Sharp(str),
        "＞" => Token::Greater(str),
        "≫" => Token::GreaterGreater(str),
        "≧" => Token::GreaterEqual(str),
        "＿" => Token::Underbar(str),
        "→" => Token::Arrow(str),
        _ => Token::Sentense(str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        analyze(vec![
            "＊".to_string(),
            "OnBoot".to_string(),
            "：".to_string(),
            "（".to_string(),
            "iflist、".to_string(),
            "（".to_string(),
            "現在時".to_string(),
            "）".to_string(),
            "、＜６、こんばんは。、＜１１、おはようございます。、＜１８、こんにちは。、＜２４、こんばんは。".to_string(),
            "）".to_string(),
        ]);
    }
}

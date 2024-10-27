use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Asta(String),
    Colon(String),
    At(String),
    Dollar(String),
    Cacco(String),
    Cocca(String),
    Sharp(String),
    Greater(String),
    GreaterGreater(String),
    GreaterEqual(String),
    Underbar(String),
    Arrow(String),
    Sentense(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Entry(Entry, Entry), // ＊
}

#[derive(Debug, PartialEq, Clone)]
pub enum Entry {
    Label(String),         // トークラベル名
    Content(Vec<Content>), // トーク内容
}

#[derive(Debug, PartialEq, Clone)]
pub enum Content {
    WordGroup(WordGroup),                     // ＠
    ScopeChange(String),                      // ：
    VariableDeclaration(VariableDeclaration), // ＄
    Macro(Macro),                             // マクロ展開式
    Jump(String),                             // ＞ トークラベル名
    AmbiguousSearchJump(String),              // ≫ トークラベル名（部分一致）
    TagAmbiguousSearchJump(String),           // ≧ タグ名（部分一致）
    UserSelections(UserSelection),            // ＿
    TalkWithOtherGhost(TalkWithOtherGhost),   // →
    Sentense(String),                         // 文
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclaration {
    Name(String),      // 変数名
    Value(Primitives), // 変数値
}

#[derive(Debug, PartialEq, Clone)]
pub enum Primitives {
    Number(f32),    // 数値
    String(String), // 文字列
}

#[derive(Debug, PartialEq, Clone)]
pub enum UserSelection {
    Label(String),   // ユーザ選択肢ラベル名
    Content(String), // ユーザ選択肢内容
}

#[derive(Debug, PartialEq, Clone)]
pub enum TalkWithOtherGhost {
    ScopeChange(String), // ：
    Macro(Macro),        // （[0-9０-９]）
    Sentense(String),    // 文
}

#[derive(Debug, PartialEq, Clone)]
pub enum WordGroup {
    Label(String),                   // 単語群ラベル名
    Contents(Vec<WordGroupContent>), // マクロ展開式か文字列
}

#[derive(Debug, PartialEq, Clone)]
pub enum WordGroupContent {
    Macro(Macro),            // マクロ展開式
    Definition(Vec<String>), // 文字列
}

#[derive(Debug, PartialEq, Clone)]
pub enum Macro {
    Macro(Rc<Macro>),               // マクロ展開式
    SurfaceChange(String),          // （[0-9０-９]）
    TalkCalling(String),            // トークラベル名
    FunctionCall(FunctionCall),     // 関数呼び出し
    VariableExpansion(Vec<String>), // 変数展開
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionCall {
    FunctionName(String),  // 関数名
    Argument(Vec<String>), // 関数引数
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Statement(Statement),
    Entry(Entry),
    Content(Content),
    VariableDeclaration(VariableDeclaration),
    Primitives(Primitives),
    UserSelection(UserSelection),
    TalkWithOtherGhost(TalkWithOtherGhost),
    WordGroup(WordGroup),
    WordGroupContent(WordGroupContent),
    Macro(Macro),
    FunctionCall(FunctionCall),
}

pub fn parse(tokens: Vec<String>) -> Result<Vec<Node>, String> {
    let mut stack: Vec<Node> = vec![];
    for (_, token) in tokens.into_iter().enumerate() {
        match token.as_str() {
            "＊" => {
                let prev2 = stack.pop();
                if let Some(prev) = stack.pop() {
                    match prev {
                        Node::Statement(_) => {
                            // 余分なpopを戻す
                            if let Some(prev2) = prev2 {
                                stack.push(prev2);
                            }
                            // 前段がステートメントの場合、次に来るのがトークラベル名になるはずなので、ここではそのままスタックに積む
                            stack.push(prev);
                            continue;
                        }
                        Node::Entry(entry) => match entry {
                            // 前段がトークラベル名のときに＊が来るのはエラー
                            Entry::Label(label) => {
                                Err(format!("＊ label Invalid token {:?}", label))?
                            }
                            Entry::Content(contents) => {
                                // ＊が来たので新しいブロックとして、前段のラベル・内容をステートメントとしてスタックに積む
                                stack.push(Node::Statement(Statement::Entry(
                                    Entry::Label(match prev2 {
                                        Some(Node::Entry(Entry::Label(label))) => label,
                                        Some(_) => Err(format!("Invalid token prev2={:?}", prev2))?,
                                        None => Err(format!("Invalid token prev2={:?}", prev2))?,
                                    }),
                                    Entry::Content(contents),
                                )));
                                continue;
                            }
                        },
                        // Node::Statement以外がいる場合、構築がうまくできていないのでエラー
                        _ => Err(format!("＊ Invalid token {:?}", prev))?,
                    }
                }
                // 余分なpopを戻す
                if let Some(prev2) = prev2 {
                    stack.push(prev2);
                }
                stack.push(Node::Entry(Entry::Label(token)));
            }
            // "：" => stack.push(Node::Content(Content::ScopeChange(token))),
            // "＠" => stack.push(Node::Content(Content::WordGroup(WordGroup::Label(token)))),
            _ => {
                // トークンが文
                let prev2 = stack.pop();
                if let Some(prev) = stack.pop().or(prev2.clone()) {
                    match prev {
                        // ステートメントの場合、トーク内容に追加
                        Node::Statement(statement) => match statement {
                            Statement::Entry(label, content) => {
                                if let Entry::Content(mut content) = content {
                                    content.push(Content::Sentense(token));
                                    stack.push(Node::Statement(Statement::Entry(
                                        label,
                                        Entry::Content(content),
                                    )));
                                }
                                continue;
                            }
                        },
                        Node::Entry(entry) => match entry {
                            Entry::Label(label) => {
                                // 前段がトークラベル名なので新しいトーク内容として追加
                                stack.push(Node::Statement(Statement::Entry(
                                    Entry::Label(label),
                                    Entry::Content(vec![Content::Sentense(token)]),
                                )));
                                continue;
                            }
                            Entry::Content(mut contents) => {
                                // 前段がステートメントにまとめる
                                if let Some(Node::Entry(Entry::Label(label))) = prev2 {
                                    contents.push(Content::Sentense(token));
                                    stack.push(Node::Statement(Statement::Entry(
                                        Entry::Label(label),
                                        Entry::Content(contents),
                                    )));
                                }
                                continue;
                            }
                        },
                        _ => Err(format!("_ not match {:?}", prev))?,
                    }
                }

                // 前回読みだしたトークンがエントリーラベルでない場合エラー
                Err(format!("Invalid token"))?
            }
        }
    }

    Ok(stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tokens = lexer::analyze(
            r"
            ＊
            こんにちは。
            ",
        )
        .unwrap();

        let results = parse(tokens).unwrap();

        println!("test output:{:?}", results);

        for result in results {
            assert_eq!(
                result,
                Node::Statement(Statement::Entry(
                    Entry::Label("＊".to_string()),
                    Entry::Content(vec![Content::Sentense("こんにちは。".to_string())]),
                ))
            );
        }
    }

    #[test]
    fn test2() {
        let tokens = lexer::analyze(
            r"
            ＊
            こんにちは。
            ＊
            ",
        )
        .unwrap();

        let results = parse(tokens).unwrap();

        println!("test output:{:?}", results);

        assert_eq!(
            results.get(0),
            Some(&Node::Statement(Statement::Entry(
                Entry::Label("＊".to_string()),
                Entry::Content(vec![Content::Sentense("こんにちは。".to_string())]),
            )))
        );

        assert_eq!(
            results.get(1),
            Some(&Node::Entry(Entry::Label("＊".to_string())))
        );
    }
}

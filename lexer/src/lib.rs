use character::Character;

mod character;

#[derive(Debug)]
enum Mode {
    Normal,
    Cacco(usize),
}

pub fn analyze(str: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut mode = Mode::Normal;

    // スキップカウント
    let mut skip = 0;
    for (idx, c) in str
        .chars()
        .enumerate()
        .map(|(idx, c)| (idx, Character::new(c)))
    {
        // スキップフラグが立っている場合はスキップ
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match mode {
            Mode::Normal => {
                normal_lexer(&mut mode, &mut tokens, &mut token, &mut skip, idx, c, str)?;
            }
            Mode::Cacco(_) => {
                cacco_lexer(&mut mode, &mut tokens, &mut token, &mut skip, idx, c, str)?;
            }
        }
    }

    if token.len() > 0 {
        tokens.push(token);
    }

    Ok(tokens)
}

fn normal_lexer(
    mode: &mut Mode,
    tokens: &mut Vec<String>,
    token: &mut String,
    skip: &mut usize,
    idx: usize,
    c: Character,
    str: &str,
) -> Result<(), String> {
    if c.is_escape() {
        // エスケープ文字が来た場合は次の文字が許可された文字なら追加
        let Some(n) = str
            .chars()
            .nth(idx + 1)
            .map(|n| Character::new(n))
            .filter(|n| n.is_allowed_escape_next())
        else {
            Err(format!(
                "エスケープ文字の後に不正な文字が続いています: idx={}, str={}",
                idx + 1,
                str[idx..10].to_string()
            ))?
        };

        // lfはスキップ
        if n.is_lf() {
            *skip += 1;
        }
    } else if c.is_spacer() {
        // 空白、改行、復帰は無視
        if c.is_cr()
            && str
                .chars()
                .nth(idx + 1)
                .map(|n| Character::new(n).is_lf())
                .unwrap_or(false)
        {
            // CRLFの場合はスキップ1回
            *skip += 1;
        }
    } else if c.is_cacco() {
        // カッコがはじまったらカッコモードで字句解析する
        *mode = Mode::Cacco(1);
        if token.len() > 0 {
            tokens.push(token.clone());
            token.clear();
        }
        tokens.push(c.to_string());
    } else if c.is_reserved() {
        // 予約語が来たのでいままでのトークンをプッシュして、予約語もプッシュ
        if token.len() > 0 {
            tokens.push(token.clone());
            token.clear();
        }
        tokens.push(c.to_string());
    } else {
        // 予約語以外はトークンに追加
        token.push(c.take());
    }

    Ok(())
}

fn cacco_lexer(
    mode: &mut Mode,
    tokens: &mut Vec<String>,
    token: &mut String,
    skip: &mut usize,
    idx: usize,
    c: Character,
    str: &str,
) -> Result<(), String> {
    if c.is_cacco() {
        let Mode::Cacco(n) = *mode else {
            Err(format!(
                "カッコがない場所でカッコ解析モードになっています: idx={}, str={}",
                idx,
                str[idx..10].to_string()
            ))?
        };
        // カッコが来たらカッコモードの深さを増やす
        *mode = Mode::Cacco(n + 1);

        if token.len() > 0 {
            tokens.push(token.clone());
            token.clear();
        }
        tokens.push(c.to_string());
    } else if c.is_cocca() {
        // カッコが閉じたらカッコモードの深さを減らす
        match *mode {
            Mode::Cacco(n) => {
                if n > 1 {
                    *mode = Mode::Cacco(n - 1);
                } else {
                    *mode = Mode::Normal;
                }
            }
            _ => Err(format!(
                "閉じカッコが不正です: idx={}, str={}",
                idx,
                str[idx..10].to_string()
            ))?,
        }
        if token.len() > 0 {
            tokens.push(token.clone());
            token.clear();
        }
        tokens.push(c.to_string());
    } else if c.is_splitter() {
        // 区切り文字が来たらトークンに追加
        if token.len() > 0 {
            tokens.push(token.clone());
            token.clear();
        }
        tokens.push(c.to_string());
    } else if c.is_escape() {
        // エスケープ文字が来た場合は次の文字が許可された文字なら追加
        let Some(mut n) = str
            .chars()
            .nth(idx + 1)
            .map(|n| Character::new(n))
            .filter(|n| n.is_allowed_escape_next())
        else {
            Err(format!(
                "エスケープ文字の後に不正な文字が続いています: idx={}, str={}",
                idx + 1,
                str[idx..10].to_string()
            ))?
        };

        // スペースはスキップ
        while n.is_spacer() {
            *skip += 1;
            n = if let Some(nn) = str
                .chars()
                .nth(idx + *skip)
                .map(|n| Character::new(n))
                .filter(|n| n.is_allowed_escape_next())
            {
                nn
            } else {
                Err(format!(
                    "エスケープ文字の後に不正な文字が続いています: idx={}, str={}",
                    idx + *skip,
                    str[idx..10].to_string()
                ))?
            }
        }

        tokens.push(n.to_string());
    } else if c.is_spacer() {
        // 空白、改行、復帰は無視
        if c.is_cr()
            && str
                .chars()
                .nth(idx + 1)
                .map(|n| Character::new(n).is_lf())
                .unwrap_or(false)
        {
            // CRLFの場合はスキップ1回
            *skip += 1;
        }
    } else if c.is_operator() {
        // 条件式が来たのでいままでのトークンをプッシュして、条件式もプッシュ
        if token.len() > 0 {
            tokens.push(token.clone());
            token.clear();
        }
        tokens.push(c.to_string());
    } else if c.is_reserved() {
        Err(format!(
            "予約語が不正な位置にあります: idx={}, str={}",
            idx,
            str[idx..10].to_string()
        ))?
    } else {
        // 予約語以外はトークンに追加
        token.push(c.take());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_analyze1() {
        let tokens = analyze(
            r"
            ＊OnBoot
            ：Φ
            （iflist、（現在時）、
            ＜６、こんばんは。、
            ＜１１、おはようございます。、
            ＜１８、こんにちは。、
            ＜２４、こんばんは。
            ）
        ",
        );

        assert_eq!(tokens.is_ok(), true);
        assert_eq!(
            tokens.unwrap(),
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
        );
    }

    #[test]
    pub fn test_analyze2() {
        let tokens = analyze(
            r"
            （へ
            んす


            う）
        ",
        );

        assert_eq!(tokens.is_ok(), true);
        assert_eq!(tokens.unwrap(), vec!["（", "へんすう", "）"]);
    }

    #[test]
    pub fn condition_talk_test() {
        let tokens = analyze(
            r"
            ＊OnBoot （現在曜日）==0
            ",
        );

        assert_eq!(tokens.is_ok(), true);
        assert_eq!(
            tokens.unwrap(),
            vec!["＊", "OnBoot", "（", "現在曜日", "）", "==0"]
        );
    }
}

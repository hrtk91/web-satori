use character::Character;

mod character;

pub fn analyze(str: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();

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

        if c.is_escape() {
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
                skip += 1;
                n = if let Some(nn) = str
                    .chars()
                    .nth(idx + skip)
                    .map(|n| Character::new(n))
                    .filter(|n| n.is_allowed_escape_next())
                {
                    nn
                } else {
                    Err(format!(
                        "エスケープ文字の後に不正な文字が続いています: idx={}, str={}",
                        idx + skip,
                        str[idx..10].to_string()
                    ))?
                }
            }

            tokens.push(n.to_string());
            continue;
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
                skip = 1;
            }
            continue;
        } else if c.is_reserved() {
            // 予約語が来たのでいままでのトークンをプッシュして、予約語もプッシュ
            if token.len() > 0 {
                tokens.push(token.clone());
                token.clear();
            }
            tokens.push(c.to_string());
            continue;
        } else {
            // 予約語以外はトークンに追加
            token.push(c.take());
            continue;
        }
    }

    Ok(tokens)
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
                "iflist、",
                "（",
                "現在時",
                "）",
                "、＜６、こんばんは。、＜１１、おはようございます。、＜１８、こんにちは。、＜２４、こんばんは。",
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
}

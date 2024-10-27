use token::*;

mod token;

pub fn parse(tokens: Vec<String>) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

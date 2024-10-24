#[derive(Debug)]
pub struct Character {
    c: char,
}

impl Character {
    pub fn new(c: char) -> Self {
        Self { c }
    }

    pub fn is_spacer(&self) -> bool {
        self.is_space() || self.is_tab() || self.is_cr() || self.is_lf()
    }

    pub fn is_space(&self) -> bool {
        self.c == ' '
    }

    pub fn is_tab(&self) -> bool {
        self.c == '\t'
    }

    pub fn is_cr(&self) -> bool {
        self.c == '\r'
    }

    pub fn is_lf(&self) -> bool {
        self.c == '\n'
    }

    /// 里々の特殊記号（予約語）か判定します
    /// ['[','＊','＠','：','＄','（','）','＃','＞','≫','≧','＿','→']
    pub fn is_reserved(&self) -> bool {
        [
            '[', '＊', '＠', '：', '＄', '（', '）', '＃', '＞', '≫', '≧', '＿', '→',
        ]
        .contains(&self.c)
    }

    /// 文字が'（'か判定します
    /// '（'のブロックは改行を無視する必要があり
    pub fn is_cacco(&self) -> bool {
        self.c == '（'
    }

    /// エスケープ文字か判定します
    /// Φ
    pub fn is_escape(&self) -> bool {
        self.c == 'Φ'
    }

    /// エスケープ文字の次に許可される文字か判定します
    pub fn is_allowed_escape_next(&self) -> bool {
        [
            'Φ', '＊', '＠', '：', '＄', '（', '）', '＃', '＞', '≫', '≧', '＿', '→', '\r', '\n',
            ' ',
        ]
        .contains(&self.c)
    }

    ///

    /// 文字を取得します
    pub fn take(self) -> char {
        self.c
    }
}

impl ToString for Character {
    fn to_string(&self) -> String {
        self.c.to_string()
    }
}

pub const BLOCK: &str = " ░▒▓█";
pub const CHINESE: &str = "\u{3000}一二十人丁口王日木金華爱黑墨龍龘";
#[rustfmt::skip]
pub const DEFAULT: &str = "  .`^\"\\,:;Il!i><~+_-?][}{1)(|\\\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B$@";
pub const EMOJI: &str = "   。，🧔👶🗣👥👤👀👁🦴🦷🫁🫀🧠👃🦻👂👅🦀👿🦀👄🤳💅🖖👆🙏🤝🦿🦾💪🤏👌🤘🤞👊🤚🤛🙌😾😿🙀😺👾👽👻💀👺🦀👹🤡💤😴🥸🥳🥶🥵🤮🤢🤕😭😓😯😰😨😱😮😩😫🙁😔😡🤬😠🙄😐😶🧐😛🤗🤐🤑😝🤩😋😊😉🤣😅😆";
pub const RUSSIAN: &str = "  ЯЮЭЬЫЪЩШЧЦХФУТСPПОНМЛКЙИЗЖЁЕДГВБА";
pub const SLIGHT: &str = "  .`\"\\:I!>~_?[{)|\\\\YLpda*W8%@$";

#[derive(Debug, Clone)]
pub enum Charset<'a> {
    Block,
    Chinese,
    Default,
    Emoji,
    Russian,
    Slight,
    Custom(&'a str),
}

impl<'a> Charset<'a> {
    #[allow(dead_code)]
    pub fn new(s: &'a str) -> Self {
        Self::Custom(s)
    }

    #[allow(dead_code)]
    pub fn from_str(s: &'a str) -> Self {
        match s {
            "block" => Charset::Block,
            "chinese" => Charset::Chinese,
            "default" => Charset::Default,
            "emoji" => Charset::Emoji,
            "russian" => Charset::Russian,
            "slight" => Charset::Slight,
            _ => Charset::Custom(s),
        }
    }
}

/// `"block".into()` -> `Charset::Block`
impl<'a> From<&'a str> for Charset<'a> {
    fn from(s: &'a str) -> Self {
        Self::from_str(s)
    }
}

/// `Charset::Block.into()` -> `" ░▒▓█"`
impl<'a> Into<&'a str> for &Charset<'a> {
    fn into(self) -> &'a str {
        match self {
            Charset::Block => BLOCK,
            Charset::Chinese => CHINESE,
            Charset::Default => DEFAULT,
            Charset::Emoji => EMOJI,
            Charset::Russian => RUSSIAN,
            Charset::Slight => SLIGHT,
            Charset::Custom(s) => s,
        }
    }
}

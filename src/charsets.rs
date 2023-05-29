pub const BLOCK: &str = " ░▒▓█";
#[rustfmt::skip]
pub const DEFAULT: &str = "  .`^\"\\,:;Il!i><~+_-?][}{1)(|\\\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B$@";
pub const EMOJI: &str = "   。，🧔👶🗣👥👤👀👁🦴🦷🫁🫀🧠👃🦻👂👅🦀👿🦀👄🤳💅🖖👆🙏🤝🦿🦾💪🤏👌🤘🤞👊🤚🤛🙌😾😿🙀😺👾👽👻💀👺🦀👹🤡💤😴🥸🥳🥶🥵🤮🤢🤕😭😓😯😰😨😱😮😩😫🙁😔😡🤬😠🙄😐😶🧐😛🤗🤐🤑😝🤩😋😊😉🤣😅😆";
pub const RUSSIAN: &str = "  ЯЮЭЬЫЪЩШЧЦХФУТСPПОНМЛКЙИЗЖЁЕДГВБА";
pub const SLIGHT: &str = "  .`\"\\:I!>~_?[{)|\\\\YLpda*W8%@$";

pub fn from_str(s: &str) -> Option<&str> {
    match s {
        "block" => Some(BLOCK),
        "default" => Some(DEFAULT),
        "emoji" => Some(EMOJI),
        "russian" => Some(RUSSIAN),
        "slight" => Some(SLIGHT),
        _ => None,
    }
}

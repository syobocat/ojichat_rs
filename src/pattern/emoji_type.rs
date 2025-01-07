use std::{collections::HashMap, sync::LazyLock};
pub static EMOJI_TYPE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut emoji_type = HashMap::new();
    emoji_type.insert("emoji_pos", "EMOJI_POS");
    emoji_type.insert("emoji_neg", "EMOJI_NEG");
    emoji_type.insert("emoji_neut", "EMOJI_NEUT");
    emoji_type.insert("emoji_ask", "EMOJI_ASK");
    emoji_type
});

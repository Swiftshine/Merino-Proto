pub struct EmojiMessage;

impl EmojiMessage {
    pub fn add_msg(msg: &str) -> String {
        format!("➕ {msg}")
    }

    // pub fn discard_msg(msg: &str) -> String {
    //     format!("🗑 {msg}")
    // }

    pub fn discard() -> &'static str {
        "🗑"
    }

    pub fn palette_msg(msg: &str) -> String {
        format!("🎨 {msg}")
    }

    pub fn memo_msg(msg: &str) -> String {
        format!("📝 {msg}")
    }

    // pub fn settings_msg(msg: &str) -> String {
    //     format!("⚙️ {msg}")
    // }

    pub fn warning_msg(msg: &str) -> String {
        format!("⚠ {msg}")
    }
}

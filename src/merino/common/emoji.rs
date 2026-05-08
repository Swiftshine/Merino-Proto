pub struct EmojiMessage;

impl EmojiMessage {
    pub fn add_msg(msg: &str) -> String {
        format!("➕ {msg}")
    }

    pub fn check_msg(msg: &str) -> String {
        format!("✔️ {msg}")
    }

    pub fn cross_msg(msg: &str) -> String {
        format!("❌ {msg}")
    }

    pub fn discard() -> &'static str {
        "🗑"
    }

    // pub fn discard_msg(msg: &str) -> String {
    //     format!("🗑 {msg}")
    // }

    pub fn memo_msg(msg: &str) -> String {
        format!("📝 {msg}")
    }

    pub fn palette_msg(msg: &str) -> String {
        format!("🎨 {msg}")
    }

    // pub fn settings_msg(msg: &str) -> String {
    //     format!("⚙️ {msg}")
    // }

    pub fn target() -> &'static str {
        "🎯"
    }

    pub fn target_msg(msg: &str) -> String {
        format!("🎯 {msg}")
    }

    pub fn warning_msg(msg: &str) -> String {
        format!("⚠ {msg}")
    }
}

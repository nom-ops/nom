pub const BG_PRIMARY: &str = "\x1b[48;2;10;10;10m";
pub const TEXT_ACCENT: &str = "\x1b[38;2;255;255;255m";
pub const TEXT_MUTED: &str = "\x1b[38;2;115;115;115m";
pub const BORDER_SUBTLE: &str = "\x1b[38;2;31;31;31m";
pub const TEXT_ERROR: &str = "\x1b[38;2;245;245;245m";
pub const RESET: &str = "\x1b[0m";

pub fn format_error_box(msg: &str, count: u32) -> String {
    format!(
        "{}{} {} {}\n{}{}┌─────────────────────────┐{}\n{}{}│ {:<23} │{}\n{}{}│ Mistake #{:<16} │{}\n{}{}└─────────────────────────┘{}",
        BG_PRIMARY, BORDER_SUBTLE, TEXT_ACCENT, "✕ ERROR", RESET,
        BORDER_SUBTLE, RESET,
        BORDER_SUBTLE, TEXT_ERROR, msg, RESET,
        BORDER_SUBTLE, TEXT_MUTED, count, RESET,
        BORDER_SUBTLE, RESET
    )
}

pub fn format_success(msg: &str) -> String {
    format!("{}{}✓ {}{}", TEXT_ACCENT, BORDER_SUBTLE, msg, RESET)
}

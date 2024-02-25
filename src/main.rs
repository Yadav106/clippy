use clippy::ClipboardHistory;

fn main() {
    let mut cb_history = ClipboardHistory::new();
    cb_history.get_clipboard_data();
}

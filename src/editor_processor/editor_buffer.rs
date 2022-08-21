pub struct EditorBuffer {
    editor_contents: String,
}

impl EditorBuffer {
    fn new() -> Self {
        Self {
            editor_contents: String::new(),
        }
    }

    pub fn push_char(self: &mut Self, ch: char) {
        self.editor_contents.push(ch);
    }

    pub fn push_str(self: &mut Self, buf: &str) {
        self.editor_contents.push_str(buf);
    }
}

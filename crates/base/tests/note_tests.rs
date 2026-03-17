// crates/base/tests/note_tests.rs

use base::Note;

#[test]
fn test_create_note_from_markdown() {
    let content = r#"---
date: 2026-03-16T10:00:00Z
---

This is a quick note without a title."#;

    let note = Note::from_markdown(content).expect("Failed to parse note");

    assert_eq!(note.body(), "This is a quick note without a title.");
}

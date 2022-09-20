use coloring::*;

#[test]
fn fg() {
    let fg = Formatting::new().foreground(Color::Blue).apply_to("text");
    assert_eq!(fg, "\x1B[34mtext\x1B[0m");
}

#[test]
fn bg() {
    let bg = Formatting::new().background(Color::Blue).apply_to("text");
    assert_eq!(bg, "\x1B[44mtext\x1B[0m");
}

#[test]
fn style() {
    let styled = Formatting::new().styles(vec![Styles::Bold]).apply_to("text");
    assert_eq!(styled, "\x1B[1mtext\x1B[0m");
}

#[test]
fn styles() {
    let styled = Formatting::new().styles(vec![Styles::Bold, Styles::Invert]).apply_to("text");
    assert_eq!(styled, "\x1B[1;7mtext\x1B[0m");
}

#[test]
fn fg_bg() {
    let formatted = Formatting::new().foreground(Color::Blue).background(Color::Blue).apply_to("text");
    assert_eq!(formatted, "\x1B[34;44mtext\x1B[0m");
}

#[test]
fn fg_style() {
    let formatted = Formatting::new().foreground(Color::Blue).styles(vec![Styles::Bold]).apply_to("text");
    assert_eq!(formatted, "\x1B[34;1mtext\x1B[0m");
}

#[test]
fn bg_style() {
    let formatted = Formatting::new().background(Color::Blue).styles(vec![Styles::Bold]).apply_to("text");
    assert_eq!(formatted, "\x1B[44;1mtext\x1B[0m");
}

#[test]
fn all() {
    let formatted = Formatting::new().foreground(Color::Blue).background(Color::Blue).styles(vec![Styles::Bold]).apply_to("text");
    assert_eq!(formatted, "\x1B[34;44;1mtext\x1B[0m");
}

#[test]
fn none() {
    let formatted = Formatting::new().apply_to("text");
    assert_eq!(formatted, "\x1B[mtext\x1B[0m");
}

//! # Coloring
//! 
//! `coloring` provides basic coloring and styling for text printed to a terminal.
//! For instance, you can make bold text with a flashing red background.
//! 
//! If you are unfamiliar with coloring / styling, some references I found useful are:
//!   - [Terminal Colors](https://chrisyeh96.github.io/2020/03/28/terminal-colors.html)
//!   - [ANSI Escape Sequences](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#colors--graphics-mode)
//!   - [Colors in Terminal](http://jafrog.com/2013/11/23/colors-in-terminal.html)
//!   - [Terminal Colors](https://github.com/termstandard/colors) (Yes, it's different.)
//! 
//! If you are unfamiliar and didn't read any of those links, just be aware that different terminals
//! have different levels of support for coloring / styling.
//! 
//! The basic use pattern is to create a `Formatting` object, call any combination of (`foreground`,
//! `background`, and `styles`), then call `apply_to` with your text as an argument. e.g.:
//! 
//! ```
//! use coloring::*;
//! 
//! let formatted_text = Formatting::new().foreground(Color::Green).styles(vec![Styles::Bold, Styles::Blink]).apply_to("HI MOM!");
//! println!("{}", formatted_text);
//! ```

/// Color options to pass to either [`foreground`] or [`background`].
/// 
/// All terminals should support the basic 8 colors at least.
/// For other options (e.g. RGB), you'll have to just check if they work for you.
/// It'll probably be fastest just to run through the options with commands in your terminal such as:
/// 
/// `printf "\x1B[34;3mtest\x1B[0ming"`
/// 
/// See the links in the module-level documentation for details.
/// 
/// Handy reference: [Color chart](https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg) for Colors256.
/// 
/// [`foreground`]: struct.Formatting.html#method.foreground
/// 
/// [`background`]: struct.Formatting.html#method.background
#[derive(Clone, Copy, Default, Debug)]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Colors256(u8),
    RGB {r: u8, g: u8, b: u8},
}

/// Style options to pass to [`styles`].
/// 
/// You'll have to just check and see which styles work on your terminal.
/// It'll probably be fastest just to run through the options with commands in your terminal such as:
/// 
/// `printf "\x1B[34;3mtest\x1B[0ming"`.
/// 
/// See the links in the module-level documentation for details.
/// 
/// **WARNING** `Styles::Reset` resets all preceding styles *and* colors, which you probably don't want.
/// 
/// [`styles`]: struct.Formatting.html#method.styles

#[derive(Clone, Copy, Debug)]
pub enum Styles {
    Reset = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Invert = 7,
    Invisible = 8,
    Strikethrough = 9,
}

enum TensDigit {
    FG = 3,
    BG = 4,
}

#[derive(Default, Debug)]
pub struct Formatting {
    fg: Color,
    bg: Color,
    styles: Option<Vec<Styles>>
}

impl Formatting {
    /// Create a new, default `Formatting` object.
    pub fn new() -> Formatting {
        Default::default()
    }

    /// Set the foreground color to `Color`.
    /// 
    /// # Example
    /// 
    /// ```
    /// use coloring::*;
    /// 
    /// let formatted_text = Formatting::new().foreground(Color::Blue).apply_to("HI MOM");
    /// println!("{}", formatted_text);
    /// ```
    pub fn foreground(&mut self, fg: Color) -> &mut Formatting {
        self.fg = fg;
        self
    }

    /// Set the background color to `Color`.
    /// 
    /// # Example
    /// 
    /// ```
    /// use coloring::*;
    /// 
    /// let formatted_text = Formatting::new().background(Color::Blue).apply_to("HI MOM");
    /// println!("{}", formatted_text);
    /// ```
    pub fn background(&mut self, bg: Color) -> &mut Formatting {
        self.bg = bg;
        self
    }

    /// Set the styles to a vector of `Styles`.
    /// 
    /// # Example
    /// 
    /// ```
    /// use coloring::*;
    /// 
    /// let formatted_text = Formatting::new().styles(vec![Styles::Bold, Styles::Blink]).apply_to("HI MOM");
    /// println!("{}", formatted_text);
    /// ```
    pub fn styles(&mut self, styles: Vec<Styles>) -> &mut Formatting {
        self.styles = Some(styles);
        self
    }
    
    /// Apply your colors and styles to text.
    /// 
    /// # Example
    /// 
    /// ```
    /// use coloring::*;
    /// let formatted_text = Formatting::new().foreground(Color::Green).styles(vec![Styles::Bold, Styles::Blink]).apply_to("HI MOM!");
    /// println!("{}", formatted_text);
    /// ```
    pub fn apply_to(&self, text: &str) -> String {
        let mut colored = "\x1B[".to_string();  // Starting delimiter.
        colored.push_str(&self.translate());
        colored.push('m');
        colored.push_str(text);
        colored.push_str("\x1B[0m");  // Ending, resetting delimiter.
        colored
    }

    fn translate(&self) -> String {
        let fg = self.translate_foreground();
        let bg = self.translate_background();
        let styles = self.translate_styles();

        let mut translation = String::new();
        if fg.is_some() {
            translation.push_str(&fg.unwrap());
            if bg.is_some() || styles.is_some() {
                translation.push(';');
            }
        }
        if bg.is_some() {
            translation.push_str(&bg.unwrap());
            if styles.is_some() {
                translation.push(';');
            }
        }
        translation.push_str(&styles.unwrap_or("".to_string()));
        translation
    }

    fn translate_colors(color: Color, tens_digit: TensDigit) -> Option<String> {
        let td = tens_digit as u8;
        match color {
            Color::Default => None,
            Color::Colors256(val) => Some(format!("{}8;5;{}", td, val)),
            Color::RGB { r, g, b } => Some(format!("{}8;2;{};{};{}", td, r, g, b)),
            Color::Black => Some(format!("{}", (10 * td) + 0)),
            Color::Red => Some(format!("{}", (10 * td) + 1)),
            Color::Green => Some(format!("{}", (10 * td) + 2)),
            Color::Yellow => Some(format!("{}", (10 * td) + 3)),
            Color::Blue => Some(format!("{}", (10 * td) + 4)),
            Color::Magenta => Some(format!("{}", (10 * td) + 5)),
            Color::Cyan => Some(format!("{}", (10 * td) + 6)),
            Color::White => Some(format!("{}", (10 * td) + 7)),
            Color::BrightBlack => Some(format!("{}", (10 * td) + 60)),
            Color::BrightRed => Some(format!("{}", (10 * td) + 61)),
            Color::BrightGreen => Some(format!("{}", (10 * td) + 62)),
            Color::BrightYellow => Some(format!("{}", (10 * td) + 63)),
            Color::BrightBlue => Some(format!("{}", (10 * td) + 64)),
            Color::BrightMagenta => Some(format!("{}", (10 * td) + 65)),
            Color::BrightCyan => Some(format!("{}", (10 * td) + 66)),
            Color::BrightWhite => Some(format!("{}", (10 * td) + 67)),
        }
    }

    fn translate_foreground(&self) -> Option<String> {
        Formatting::translate_colors(self.fg, TensDigit::FG)
    }

    fn translate_background(&self) -> Option<String> {
        Formatting::translate_colors(self.bg, TensDigit::BG)
    }

    fn translate_styles(&self) -> Option<String> {
        match &self.styles {
            None => None,
            Some(styles) => {
                let styles: Vec<String> = styles.iter().map(|&x| (x as u8).to_string()).collect();
                Some(styles.join(";"))
            }
        }
    }
}

use nu_ansi_term::Style;

use {crate::styled_text::StyledText, nu_ansi_term::Color};

pub static DEFAULT_BUFFER_MATCH_COLOR: Color = Color::Green;
pub static DEFAULT_BUFFER_NEUTRAL_COLOR: Color = Color::White;
pub static DEFAULT_BUFFER_NOTMATCH_COLOR: Color = Color::Red;

/// The syntax highlighting trait. Implementers of this trait will take in the current string and then
/// return a `StyledText` object, which represents the contents of the original line as styled strings
pub trait Highlighter {
    /// The action that will handle the current buffer as a line and return the corresponding `StyledText` for the buffer
    fn highlight(&self, line: &str) -> StyledText;
}

/// A simple, example highlighter that shows how to highlight keywords
pub struct DefaultHighlighter {
    external_commands: Vec<String>,
    match_color: Color,
    notmatch_color: Color,
    neutral_color: Color,
}

impl Highlighter for DefaultHighlighter {
    fn highlight(&self, line: &str) -> StyledText {
        let mut styled_text = StyledText::new();

        if self
            .external_commands
            .clone()
            .iter()
            .any(|x| line.contains(x))
        {
            let matches: Vec<&str> = self
                .external_commands
                .iter()
                .filter(|c| line.contains(*c))
                .map(std::ops::Deref::deref)
                .collect();
            let longest_match = matches.iter().fold("".to_string(), |acc, &item| {
                if item.len() > acc.len() {
                    item.to_string()
                } else {
                    acc
                }
            });
            let buffer_split: Vec<&str> = line.splitn(2, &longest_match).collect();

            styled_text.push((
                Style::new().fg(self.neutral_color),
                buffer_split[0].to_string(),
            ));
            styled_text.push((Style::new().fg(self.match_color), longest_match));
            styled_text.push((
                Style::new().bold().fg(self.neutral_color),
                buffer_split[1].to_string(),
            ));
        } else if self.external_commands.is_empty() {
            styled_text.push((Style::new().fg(self.neutral_color), line.to_string()));
        } else {
            styled_text.push((Style::new().fg(self.notmatch_color), line.to_string()));
        }

        styled_text
    }
}
impl DefaultHighlighter {
    /// Construct the default highlighter with a given set of extern commands/keywords to detect and highlight
    pub fn new(external_commands: Vec<String>) -> DefaultHighlighter {
        DefaultHighlighter {
            external_commands,
            match_color: DEFAULT_BUFFER_MATCH_COLOR,
            notmatch_color: DEFAULT_BUFFER_NOTMATCH_COLOR,
            neutral_color: DEFAULT_BUFFER_NEUTRAL_COLOR,
        }
    }

    /// Configure the highlighter to use different colors
    pub fn change_colors(
        &mut self,
        match_color: Color,
        notmatch_color: Color,
        neutral_color: Color,
    ) {
        self.match_color = match_color;
        self.notmatch_color = notmatch_color;
        self.neutral_color = neutral_color;
    }
}
impl Default for DefaultHighlighter {
    fn default() -> Self {
        DefaultHighlighter::new(vec![])
    }
}

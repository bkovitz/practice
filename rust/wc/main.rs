#[derive(Debug, PartialEq)]
struct TextStats<'a> {
    s: &'a str,
    num_words: usize,
    num_lines: usize,
}

impl<'a> TextStats<'a> {
    fn summary(&self) -> &str {
        match self.s.find('\n') {
            None => self.s,
            Some(i) => &self.s[..i],
        }
    }

    fn preview(&self) -> String {
        let mut state = State::NotInWord;
        let mut after_4_words = self.s.len();
        let mut word_num: u32 = 0;
        for (i, c) in self.s.char_indices() {
            match state {
                State::NotInWord => {
                    if !c.is_whitespace() {
                        word_num += 1;
                        state = State::InWord;
                    }
                }
                State::InWord => {
                    if c.is_whitespace() {
                        if word_num >= 4 {
                            after_4_words = i;
                            break;
                        }
                        state = State::NotInWord;
                    }
                }
            }
        }
        format!("{}...", &self.s[..after_4_words])
    }
}

trait Reportable {
    fn report(&self) -> String;
}

enum State {
    NotInWord,
    InWord
}

impl<'a> Reportable for TextStats<'a> {
    fn report(&self) -> String {
        format!("Lines: {}\nWords: {}\nPreview: {}\n", self.num_lines, self.num_words, self.preview())
    }
}

fn print_report<T: Reportable>(r: &T) {
    println!("{}", r.report());
}

impl<'a> TextStats<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut state = State::NotInWord;
        let mut num_words: usize = 0;
        let mut num_lines: usize = 0;

        for c in s.chars() {
            if c == '\n' {
                num_lines += 1;
            }
            match state {
                State::NotInWord => {
                    if !c.is_whitespace() {
                        num_words += 1;
                        state = State::InWord;
                    }
                }
                State::InWord => {
                    if c.is_whitespace() {
                        state = State::NotInWord;
                    }
                }
            }
        }
        if s.len() > 0 && !s.ends_with('\n') {
            num_lines += 1;
        }
        TextStats { s, num_words, num_lines }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_lines() {
        let text_stats = TextStats::new("If lose mittens, get\nno pie.\n");
        assert_eq!(text_stats, TextStats {s: "If lose mittens, get\nno pie.\n", num_lines: 2, num_words: 6});
        assert_eq!(text_stats.summary(), "If lose mittens, get");
        assert_eq!(text_stats.report(), "Lines: 2
Words: 6
Preview: If lose mittens, get...\n")
    }
    
    #[test]
    fn test_2_words() {
        let text_stats = TextStats::new("Two words.");
        assert_eq!(text_stats, TextStats {s: "Two words.", num_lines: 1, num_words: 2});
        assert_eq!(text_stats.summary(), "Two words.");
        assert_eq!(text_stats.preview(), "Two words....");
    }

    #[test]
    fn test_5_words() {
        let text_stats = TextStats::new("This one is five words.");
        assert_eq!(text_stats, TextStats {s: "This one is five words.", num_lines: 1, num_words: 5});
        assert_eq!(text_stats.preview(), "This one is five...");
    }

    #[test]
    fn test_4_words() {
        let text_stats = TextStats::new("This one is four.");
        assert_eq!(text_stats, TextStats {s: "This one is four.", num_lines: 1, num_words: 4});
        assert_eq!(text_stats.preview(), "This one is four....");
    }

    #[test]
    fn test_empty() {
        let text_stats = TextStats::new("");
        assert_eq!(text_stats, TextStats {s: "", num_lines: 0, num_words: 0});
        assert_eq!(text_stats.preview(), "...");
    }

    #[test]
    fn test_partial_line() {
        let text_stats = TextStats::new("Line with terminator\nLine without terminator");
        assert_eq!(text_stats, TextStats {s: "Line with terminator\nLine without terminator", num_lines: 2, num_words: 6});
    }

}

fn main () {
    let text_stats = TextStats::new("If lose mittens, get\nno pie.\n");
    println!("{:#?}", text_stats);
    println!("{}", text_stats.summary());
    print_report(&text_stats);
}
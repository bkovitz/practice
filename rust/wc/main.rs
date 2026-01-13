#[derive(Debug)]
struct TextStats<'a> {
    s: &'a str,
    num_words: usize,
    num_lines: usize,
}

impl<'a> TextStats<'a> {
    fn summary(&self) -> &str {
        match self.s.find('\n') {
            None => self.s,
            Some(i) => &self.s[..i]
        }
    }
}

fn wc(s: &str) -> TextStats {
    enum State {
        NotInWord,
        InWord
    }
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
    TextStats { s, num_words, num_lines }
}

fn main () {
    let text_stats = wc("If lose mittens, get\nno pie.\n");
    println!("{:#?}", text_stats);
    println!("{}", text_stats.summary());
}
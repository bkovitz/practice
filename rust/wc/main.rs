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
            Some(i) => &self.s[..i],
        }
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
        format!("Lines: {}\nWords: {}\nPreview: {}...", self.num_lines, self.num_words, &self.s[..after_4_words])
    }
}

fn wc<'a>(s: &'a str) -> TextStats<'a> {
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
    if !s.ends_with('\n') {
        num_lines += 1;
    }
    TextStats { s, num_words, num_lines }
}

fn main () {
    let text_stats = wc("If lose mittens, get\nno pie.\n");
    println!("{:#?}", text_stats);
    println!("{}", text_stats.summary());
    println!("{}", text_stats.report());
    println!();
    let text_stats = wc("Two words.");
    println!("{:#?}", text_stats);
    println!("{}", text_stats.summary());
    println!("{}", text_stats.report());
    println!();
    let text_stats = wc("This one is five words.");
    println!("{:#?}", text_stats);
    println!("{}", text_stats.summary());
    println!("{}", text_stats.report());
    println!();
    let text_stats = wc("This one is four.");
    println!("{:#?}", text_stats);
    println!("{}", text_stats.summary());
    println!("{}", text_stats.report());
}
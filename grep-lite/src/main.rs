use std::collections::vec_deque::VecDeque;
use regex::Regex;

const MATCH_DELIM: char = ':';
const CTX_DELIM: char = '-';

struct Ctx {
    data: VecDeque<String>,
    capacity: usize,
}

impl Ctx {
    pub fn with_capacity(capacity: usize) -> Ctx {
        Ctx {
            data: VecDeque::with_capacity(capacity),
            capacity
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push_back(&mut self, item: &str) {
        if self.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(String::from(item));
    }
    
    pub fn get(&self, i: usize) -> Option<&str> {
        self.data.get(i).map(|x| &**x)
    }
}

struct Matcher<'a> {
    term: &'a str,
    re: Option<Regex>,
}

impl<'a> Matcher<'a> {
    pub fn new(term: &'a str, re_mode: bool) -> Matcher<'a> {
        let re = if re_mode {
            Some(Matcher::parse_re(term))
        } else { 
            None
        };

        Matcher { term, re }
    }

    pub fn matches(&self, s: &str) -> bool {
        if let Some(re) = &self.re {
            re.find(s).is_some()
        } else {
            s.contains(&self.term)
        }
    }

    fn parse_re(term: &str) -> Regex {
        Regex::new(term)
            .expect(&format!(
                    "Could not create a regular expression from {}",
                    term))
    }
}

fn main() {
    let re_mode = true;
    let need_line_num = true;
    let ctx_lines = 2;
    let need_ctx = ctx_lines > 0;
    let mut ctx: Ctx = if need_ctx {
        Ctx::with_capacity(ctx_lines) 
    } else {
        Ctx::with_capacity(0)
    };

    let search_term = "oo";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly tuned--in search of what?
It is the same with books.
What do we seek through millions of pages?
";
    let matcher = Matcher::new(search_term, re_mode);
    let mut ctx_head_offset: usize = 0; 
    let mut rem_ctx_lines = 0;
    for (i, line) in quote.lines().enumerate() {
        let is_match;
        if matcher.matches(&line) {
            is_match = true;
            rem_ctx_lines = ctx_lines;
        } else {
            is_match = false;
        }
        if is_match || rem_ctx_lines > 0 {
            if need_ctx && is_match {
                while ctx_head_offset != ctx.len() {
                    print(ctx.get(ctx_head_offset).unwrap(),
                        i - ctx_head_offset, false, need_line_num);
                    ctx_head_offset += 1;
                }
            }
            print(line, i + 1, is_match, need_line_num);
            if !is_match && need_ctx {
                rem_ctx_lines -= 1;
            }
        } else if need_ctx {
            ctx.push_back(line);
            ctx_head_offset = ctx_head_offset.saturating_sub(1);
        }
    }
}

fn print(line: &str, line_num: usize, is_match: bool, need_line_num: bool) {
    let num = if need_line_num {
        line_num.to_string()
    } else {
        "".to_string()
    };
    println!("{}{} {}", num, delim(is_match), line);
}

const fn delim(is_match: bool) -> char {
    if is_match {
        MATCH_DELIM
    } else {
        CTX_DELIM
    }
}


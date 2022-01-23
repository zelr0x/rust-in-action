use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use std::collections::vec_deque::VecDeque;
use regex::Regex;
use clap::{App, Arg, ArgMatches};

const EMPTY_STR: &str = "";
const MATCH_DELIM: &str = ":";
const CTX_DELIM: &str = "-";
const STDIN_INPUT: &str = "-";
const ARG_PATTERN: &str = "PATTERN";
const ARG_FILE: &str = "FILE";
const ARG_LINUM_ON: &str = "LINUM_ON";
const ARG_CTX: &str = "CTX_ON";
const ARG_RE_MODE: &str = "RE_MODE";

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

// cargo run -- args
// e.g.
// cargo run --oo test.txt
// or
// cat test.txt | cargo run -- oo -
fn main() {
    let args = parse_args();
    let re_mode: bool = args.value_of_t(ARG_RE_MODE).unwrap_or(true);
    let linum_on: bool = args.value_of_t(ARG_LINUM_ON).unwrap_or(false);
    let ctx_lines: usize = args.value_of_t(ARG_CTX).unwrap_or(0);
    let need_ctx = ctx_lines > 0;
    let mut ctx: Ctx = if need_ctx {
        Ctx::with_capacity(ctx_lines) 
    } else {
        Ctx::with_capacity(0)
    };
    let search_term = args.value_of(ARG_PATTERN).unwrap();
    let stdin = &io::stdin();
    let reader = new_reader(&args, stdin);
    let matcher = Matcher::new(search_term, re_mode);
    let mut ctx_head_offset: usize = 0; 
    let mut rem_ctx_lines = 0;
    for (i, line_) in reader.lines().enumerate() {
        let line = &line_.unwrap();
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
                        i - ctx_head_offset, false, linum_on);
                    ctx_head_offset += 1;
                }
            }
            print(line, i + 1, is_match, linum_on);
            if !is_match && need_ctx {
                rem_ctx_lines -= 1;
            }
        } else if need_ctx {
            ctx.push_back(line);
            ctx_head_offset = ctx_head_offset.saturating_sub(1);
        }
    }
}

fn parse_args() -> ArgMatches {
    App::new("grep-lite")
        .version("0.1")
        .about("Search for PATTERN in FILE")
        .arg(Arg::new(ARG_PATTERN)
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::new(ARG_FILE)
            .help("The file to search in")
            .takes_value(true)
            .required(true))
        .arg(Arg::new(ARG_RE_MODE)
            .help("PATTERN is a basic regular expression (default)")
            .takes_value(false)
            .required(false)
            .default_value("true")
            .long("basic-regexp")
            .short('G'))
        .arg(Arg::new(ARG_LINUM_ON)
            .help("print line number with output lines")
            .takes_value(false)
            .required(false)
            .default_value("false")
            .default_missing_value("true")
            .long("line-number")
            .short('n'))
        .arg(Arg::new(ARG_CTX)
            .help("print NUM lines of output context")
            .takes_value(true)
            .required(false)
            .default_value("0")
            .long("context")
            .short('C'))
        .get_matches() 
}

fn new_reader<'a>(args: &ArgMatches, stdin: &'a io::Stdin) -> Box<dyn BufRead + 'a> {
    let input = args.value_of(ARG_FILE).unwrap();
    if input == STDIN_INPUT {
        Box::new(stdin.lock())
    } else {
        let f = File::open(input)
            .expect(&format!("{}: No such file or directory", input));
        Box::new(BufReader::new(f))
    }
}

fn print(line: &str, line_num: usize, is_match: bool, need_line_num: bool) {
    let num: &str;
    let owned_num_str;
    let del: &str;
    if need_line_num {
        owned_num_str = line_num.to_string();
        num = &owned_num_str;
        del = delim(is_match);
    } else {
        num = EMPTY_STR;
        del = EMPTY_STR;
    }
    println!("{}{}{}", num, del, line);
}

const fn delim(is_match: bool) -> &'static str {
    if is_match {
        MATCH_DELIM
    } else {
        CTX_DELIM
    }
}


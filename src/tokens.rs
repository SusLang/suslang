
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a>(pub &'a str);

impl<'a> PartialEq<&'a str> for Token<'a> {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}


const SPLIT_CHARS: &[char] = &[' ','ච','\"','ඞ','\n','➤', '(', ')', '\t'];
const IGNORED_SEPARATORS: &[char] = &[' ', '\t'];

// චcomplete report "hello world"ඞ


pub fn tokenize(s: &str) -> Vec<Token> {
	let mut v = Vec::new();
	// let mut current = "";
	let mut start = 0;
	let mut in_str = false;

	for (char_i, (i, c)) in s.char_indices().enumerate(){
		if in_str {
			if c == '"' && s.chars().nth(char_i-1) != Some('\\') {
				v.push(Token(&s[start..i]));
				v.push(Token(&s[i..(i+c.len_utf8())]));
				start = i + c.len_utf8();
				in_str = false;
			}
		}else if SPLIT_CHARS.contains(&c) {
			if start != i {
				v.push(Token(&s[start..i]));
			}
			if !IGNORED_SEPARATORS.contains(&c) {
				v.push(Token(&s[i..(i+c.len_utf8())]));
			}
			start = i + c.len_utf8();
			if c == '"' {
				in_str = true;
			}
		}
	}
	v
}
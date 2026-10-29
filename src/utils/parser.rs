
const DOUBLE_QUOTE_ESCAPE_CHARS: [u8; 2] = [b'"', b'\\'];

pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut parsed_parts: Vec<String> = Vec::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut current_arg: Vec<u8> = Vec::new();
    let mut escape_indexes: Vec<usize> = Vec::new();
    let input = input.as_bytes();
    for i in 0..input.len() {
        let chr = input[i];
        let is_backslash = i != 0 && input[i-1] == b'\\' && !single_quote && !escape_indexes.contains(&(i-1));
        if is_backslash {
            if !double_quote || DOUBLE_QUOTE_ESCAPE_CHARS.contains(&chr) {
                escape_indexes.push(i);
                current_arg.push(chr);
            }
            continue;
        }

        if chr == b'\\' && !single_quote { continue }

        if chr == b'"' && !single_quote {
            double_quote = !double_quote;
            continue;
        }
        
        if chr == b'\'' && !double_quote {
            single_quote = !single_quote;
            continue;
        }

        if chr == b' ' && !(single_quote || double_quote) {
            if !current_arg.is_empty() {
                let bytes = std::mem::take(&mut current_arg);
                parsed_parts.push(String::from_utf8(bytes).unwrap());
            }
            continue;
        }
        current_arg.push(chr);
    }
    if !current_arg.is_empty() {
        parsed_parts.push(String::from_utf8(current_arg).unwrap());
    }

    let command = parsed_parts.remove(0);
    return (command, parsed_parts);
}
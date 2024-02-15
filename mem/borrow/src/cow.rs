use std::borrow::Cow;

// Use Cow for the return value to avoid multiple copies
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}

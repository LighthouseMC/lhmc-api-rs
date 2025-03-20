use std::borrow::Cow;


pub fn sanitise_text(text : &str) -> String {
    text.chars().map(|ch| match (ch) {
        '\\' => Cow::Borrowed("\\\\"),
        '<'  => Cow::Borrowed("\\<"),
        _    => Cow::Owned(ch.to_string())
    }).collect()
}

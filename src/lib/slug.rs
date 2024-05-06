pub fn sluggify(slug: String) -> String {
    let mut sluggified = Vec::with_capacity(slug.len());

    while let Some(b_char) = slug.bytes().next() {
        match b_char {
            b'a'..=b'z' => sluggified.push(b_char),
            b' ' => sluggified.push(b'-'),
            _ => (),
        }
    }

    std::str::from_utf8(&sluggified).unwrap().to_string()
}

pub fn pluralise<'a>(num: i32, singular_name: &'a str, plural_name: &'a str) -> &'a str {
    match num {
        1 => singular_name,
        _ => plural_name,
    }
}

pub fn n_pluralise<'a>(num: i32, singular_name: &'a str, plural_name: &'a str) -> String {
    format!("{} {}", num, pluralise(num, singular_name, plural_name))
}

pub fn n_bytes_pluralise(num: i32) -> String {
    n_pluralise(num, "byte", "bytes")
}

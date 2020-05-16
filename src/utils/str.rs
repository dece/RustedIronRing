pub fn plural<'a>(num: i32, singular_name: &'a str, plural_name: &'a str) -> &'a str {
    match num {
        1 => singular_name,
        _ => plural_name,
    }
}

pub fn n_plural<'a>(num: i32, singular_name: &'a str, plural_name: &'a str) -> String {
    format!("{} {}", num, plural(num, singular_name, plural_name))
}

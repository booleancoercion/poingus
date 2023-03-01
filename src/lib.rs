use rayon::prelude::*;

fn canonicalize_name(name: &str) -> Vec<u8> {
    name.chars()
        .filter_map(|ch| {
            ch.is_ascii_alphabetic()
                .then(|| ch.to_ascii_lowercase() as u8)
        })
        .collect()
}

pub fn get_matches<'a, S: AsRef<str> + Sync>(
    username: &str,
    nickname: Option<&str>,
    dictionary: &'a [S],
) -> Vec<&'a str> {
    let username = canonicalize_name(username);
    let nickname = nickname.map(canonicalize_name);

    dictionary
        .into_par_iter()
        .map(|s| s.as_ref())
        .filter(|word| {
            is_subsequence(word.as_bytes(), &username)
                || nickname
                    .as_deref()
                    .map_or(false, |nickname| is_subsequence(word.as_bytes(), nickname))
        })
        .collect()
}

// courtesy of orlp
fn is_subsequence(needle: &[u8], haystack: &[u8]) -> bool {
    needle
        .iter()
        .scan(0, |pos, ch| {
            *pos += 1 + haystack.iter().skip(*pos).position(|x| x == ch)?;
            Some(true)
        })
        .count()
        == needle.len()
}

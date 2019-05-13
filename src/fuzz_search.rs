use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    static ref SIMILAR_PAIRS: HashMap<char, Vec<char>> = {
        let mut s1 = HashMap::new();
        s1.insert('e', vec!['i', 'a']);
        s1.insert('i', vec!['e', 'a']);
        s1.insert('a', vec!['å', 'ä']);
        s1.insert('o', vec!['ö']);
        s1.insert('v', vec!['w']);
        s1.insert('s', vec!['z']);
        s1.insert('n', vec!['ñ']);
        s1
    };
}

pub fn fuzzy_search_score(pattern: &str, st: &str) -> isize {
    let pattern = pattern.nfc().collect::<String>();
    let st = st.nfc().collect::<String>();

    let orig_st = st.clone();

    let pattern = pattern.to_lowercase();
    let st = st.to_lowercase();

    let pat_chars = pattern.chars().collect::<Vec<char>>();
    let st_chars = st.chars().collect::<Vec<char>>();

    let mut lcs_lens = vec![None; pat_chars.len() * st_chars.len()];

    fn lcs_search(
        lcs_lens: &mut [Option<usize>],
        p_start: usize,
        s_start: usize,
        pat: &[char],
        st: &[char],
    ) {
        if p_start >= pat.len() || s_start >= st.len() {
            return;
        }

        if lcs_lens[p_start * st.len() + s_start].is_some() {
            return;
        }

        if pat[p_start] == st[s_start] {
            lcs_search(lcs_lens, p_start + 1, s_start + 1, pat, st);

            let a;
            if s_start < st.len() - 1 {
                a = match lcs_lens.get((p_start + 1) * st.len() + s_start + 1) {
                    Some(Some(x)) => *x,
                    _ => 0,
                };
            } else {
                a = 0;
            }

            lcs_lens[p_start * st.len() + s_start] = Some(a + 1);
        } else {
            lcs_search(lcs_lens, p_start + 1, s_start, pat, st);
            lcs_search(lcs_lens, p_start, s_start + 1, pat, st);

            let a;
            if s_start < st.len() - 1 {
                a = lcs_lens[p_start * st.len() + s_start + 1].unwrap_or(0);
            } else {
                a = 0;
            }

            let b = match lcs_lens.get((p_start + 1) * st.len() + s_start) {
                Some(Some(x)) => *x,
                _ => 0,
            };

            lcs_lens[p_start * st.len() + s_start] = Some(a.max(b));
        }
    }

    lcs_search(&mut lcs_lens, 0, 0, &pat_chars, &st_chars);

    // println!(" {}", st);
    // for (p_start, ch) in pat_chars.iter().enumerate() {
    //     print!("{}", ch);
    //     for s_start in 0..st_chars.len() {
    //         if let Some(i) = lcs_lens[p_start * st_chars.len() + s_start] {
    //             print!("{}", i);
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut score: isize = 0;
    let mut p_start = 0;
    let mut s_start = 0;

    let mut match_len = 1;

    while let Some(Some(curr)) = lcs_lens.get(p_start * st_chars.len() + s_start) {
        if p_start >= pat_chars.len() || s_start >= st_chars.len() {
            break;
        }
        if st_chars[s_start] == pat_chars[p_start] {
            if orig_st
                .chars()
                .nth(s_start)
                .map(|x| x.is_uppercase())
                .unwrap_or(false)
            {
                score += 2;
            }
            if match_len > 1
                && orig_st
                    .chars()
                    .nth(s_start - 1)
                    .map(|x| x.is_ascii_whitespace())
                    .unwrap_or(false)
            {
                score += 2;
            }
            p_start += 1;
            s_start += 1;
            match_len += 1;
            if match_len >= 3 {
                score += 1;
            }
        } else {
            if !SIMILAR_PAIRS
                .get(&pat_chars[p_start])
                .map(|x| x.contains(&st_chars[s_start]))
                .unwrap_or(false)
            {
                if match_len != 0 {
                    score -= 1;
                }
                match_len = 0;
            }
            let p_plus = lcs_lens.get((p_start + 1) * st_chars.len() + s_start);
            if p_plus == Some(&Some(*curr)) {
                if s_start < st_chars.len() - 1 {
                    let s_plus = lcs_lens.get((p_start + 1) * st_chars.len() + s_start + 1);
                    if s_plus == Some(&Some(*curr)) {
                        s_start += 1;
                    }
                }
                p_start += 1;
            } else {
                s_start += 1;
            }
        }
    }

    return score;
}

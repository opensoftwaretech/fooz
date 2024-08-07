pub fn fuzzy_search<'a, T>(array: &'a [T], target: &str, tolerance: usize, extract: fn(&T) -> &str) -> Vec<&'a T> {
    let mut results = Vec::with_capacity(array.len());

    for item in array {
        let item_str = extract(item);
        if fuzzy_match(item_str, target, tolerance) {
            results.push(item);
        }
    }

    results
}

pub fn fuzzy_match(s1: &str, s2: &str, tolerance: usize) -> bool {
    let len_diff = s1.len().abs_diff(s2.len());
    if len_diff > tolerance {
        return false;
    }

    let mut errors = len_diff;
    let mut s1_iter = s1.chars();
    let mut s2_iter = s2.chars();

    while let (Some(c1), Some(c2)) = (s1_iter.next(), s2_iter.next()) {
        if c1 != c2 {
            errors += 1;
            if errors > tolerance {
                return false;
            }
        }
    }

    errors <= tolerance
}

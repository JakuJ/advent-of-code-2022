/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::str::FromStr;

use regex::Regex;

pub fn parse_with_regex<T, const N: usize>(re: &Regex, line: &str) -> [T; N]
where
    T: Default,
    T: Copy,
    T: FromStr,
{
    let caps = re.captures(line).unwrap();

    let mut arr = [Default::default(); N];

    for i in 1..=N {
        unsafe {
            arr[i - 1] = caps
                .get(i)
                .unwrap()
                .as_str()
                .parse::<T>()
                .unwrap_unchecked()
        }
    }

    arr
}

pub fn vec_mut_ref<T>(v: &mut Vec<T>, a0: usize, a1: usize) -> (&mut T, &mut T) {
    assert_ne!(a0, a1, "Vector indices cannot be equal at runtime");
    // SAFETY: this is safe because we know a0 != a1
    unsafe { (&mut *(&mut v[a0] as *mut _), &mut *(&mut v[a1] as *mut _)) }
}

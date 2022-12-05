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
    let mut caps = re.capture_locations();
    re.captures_read(&mut caps, line);

    let mut arr = [Default::default(); N];

    for i in 1..=N {
        unsafe {
            let (i0, i1) = caps.get(i).unwrap_unchecked();
            let val = &line[i0..i1];

            arr[i - 1] = val.parse().unwrap_unchecked()
        }
    }

    arr
}

pub fn disjoint_mut_refs<T>(v: &mut [T], a0: usize, a1: usize) -> (&mut T, &mut T) {
    assert_ne!(a0, a1, "Vector indices cannot be equal at runtime");
    // SAFETY: this is safe because we know a0 != a1
    unsafe { (&mut *(&mut v[a0] as *mut _), &mut *(&mut v[a1] as *mut _)) }
}

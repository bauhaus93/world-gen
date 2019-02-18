use std;

#[allow(dead_code)]
pub fn format_number<T>(num: T) -> String
where T: std::fmt::Display + std::ops::Div + std::cmp::PartialOrd,
      <T as std::ops::Div>::Output: std::fmt::Display,
      f64: std::convert::From<T> {
    const PREFIXES: [(f64, &str); 3] = [
        (1000000000f64, "G"),
        (1000000f64, "M"),
        (1000f64, "k")
    ];
    let num64 = f64::from(num);
    for (threshold, prefix) in PREFIXES.iter() {
        if num64 >= *threshold {
            return format!("{:.2}{}", num64 / *threshold, prefix);
        }
    }
    format!("{}", num64)
}

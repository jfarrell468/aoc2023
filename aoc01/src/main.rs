fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let mut first = None;
        let mut last = 0;
        for c in line.unwrap().bytes() {
            if !c.is_ascii_digit() {
                continue;
            }
            if first.is_none() {
                first = Some((c - b'0') as u32);
            }
            last = (c - b'0') as u32;
        }
        sum += 10 * first.unwrap() + last;
    }
    println!("{}", sum);
}

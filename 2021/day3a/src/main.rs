use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify filename");

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let result = day3a(reader.lines().map(|line| line.unwrap()));

    println!("result: {}", result);

    Ok(())
}

fn day3a<'a, I>(lines: I) -> u32
where
    I: Iterator<Item = String>,
{
    lines
        .map(|line| (u32::from_str_radix(&*line, 2), line.len()))
        .scan([0i32; 32], |state, (x, width)| {
            let value = x.unwrap();
            let mut mask = 1u32;
            for i in 0..width {
                let is_set = value & mask != 0;
                mask <<= 1;
                if is_set {
                    state[i] += 1;
                } else {
                    state[i] -= 1;
                }
            }
            Some(state.clone())
        })
        .last()
        .map(|x| {
            let mut gamma = 0u32;
            let mut rho = 0u32;
            let mut mask = 1u32;
            for i in 0..x.len() {
                if x[i] > 0 {
                    gamma |= mask;
                } else if x[i] < 0 {
                    rho |= mask;
                }
                mask <<= 1;
            }

            gamma * rho
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day3a;

    #[test]
    fn test_day3a() {
        let lines = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let result = day3a(lines.iter().map(|x| x.to_string()));
        assert_eq!(result, 198);
    }
}

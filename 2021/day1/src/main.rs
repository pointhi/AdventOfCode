use std::collections::vec_deque::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify filename");

    println!("Read file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Start evaluation of file");
    let result = count_increases(&contents);
    println!("Number of increments: {}", result);
}

/*pub trait SlidingWindow: Iterator {
    fn window(self, window_size: usize) -> Skip<Self>
    where
        Self: Sized,
        Self::Item: Ord;
}

impl<I> SlidingWindow for I
where
    I: Iterator,
{
    fn window(self, window_size: usize) -> Skip<I>
    where
        Self: Sized,
        Self::Item: Sized,
    {
        self.scan(VecDeque::<<I as Iterator>::Item>::new(), |state, x| {
            state.push_back(x);
            if state.len() > window_size {
                state.pop_front();
            }
            Some(state.clone())
        })
        .map(|scan| scan.iter().sum())
        .skip(window_size /*-1*/)
    }
}*/

// TODO: use iterator, so we do not need to put everything in memory
fn count_increases(input: &str) -> i32 {
    struct ScanState {
        is_first: bool,
        last_elem: i32,
        increments: i32,
    }

    input
        .lines()
        .map(|line| line.parse::<i32>().expect("Cannot convert line to i32"))
        .scan(VecDeque::<i32>::new(), |state, x| {
            state.push_back(x);
            if state.len() > 3 {
                state.pop_front();
            }
            Some(scan.iter().sum())
        })
        .skip(2) // TODO: abstract the sliding window into a function which can be added to the iterator
        .scan(
            ScanState {
                is_first: true,
                last_elem: 0,
                increments: 0,
            },
            |state, cur_elem| {
                if state.is_first {
                    state.is_first = false;
                } else if state.last_elem < cur_elem {
                    state.increments += 1;
                }
                state.last_elem = cur_elem;
                Some(state.increments)
            },
        )
        .last()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::count_increases;

    #[test]
    #[should_panic]
    fn count_increases_invalid() {
        count_increases("a"); // TODO: replace with proper Result instead of panic
    }

    #[test]
    #[should_panic]
    fn count_increases_empty_newline() {
        count_increases("\n"); // TODO: replace with proper Result instead of panic
    }

    #[test]
    fn count_increases_empty() {
        assert_eq!(count_increases(""), 0);
    }

    #[test]
    fn count_increases_single() {
        assert_eq!(count_increases("100"), 0);
    }

    #[test]
    fn count_increases_same() {
        assert_eq!(count_increases("100\n100"), 0);
    }

    #[test]
    fn count_increases_increment() {
        assert_eq!(count_increases("100\n101"), 0);
    }

    #[test]
    fn count_increases_decrement() {
        assert_eq!(count_increases("100\n99"), 0);
    }

    #[test]
    fn count_increases_increment_decrement() {
        assert_eq!(count_increases("100\n101\n100"), 0);
    }

    #[test]
    fn count_increases_decrement_increment() {
        assert_eq!(count_increases("100\n99\n100"), 0);
    }

    #[test]
    fn count_increases_window() {
        assert_eq!(count_increases("1\n1\n1\n1"), 0);
        assert_eq!(count_increases("1\n1\n1\n2"), 1);
        assert_eq!(count_increases("1\n1\n1\n2\n2"), 2);
    }
}

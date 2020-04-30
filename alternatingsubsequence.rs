struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next_line(&self) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Fail to read");
        line
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let line = self.next_line();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let t: usize = scan.next();
    for _ in 0..t {
        let n: usize = scan.next();
        let arr: Vec<isize> = scan.next_n(n);
        let mut sign = arr[0].signum();
        let mut max = std::isize::MIN;
        let mut result = 0i64;
        for &item in &arr {
            if item.signum() == sign {
                max = max.max(item);
            } else {
                result += max as i64;
                max = item;
                sign = item.signum();
            }
        }
        result += max as i64;
        println!("{}", result);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}

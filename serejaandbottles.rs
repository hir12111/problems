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
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut bottles: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        let a: usize = scan.next();
        let b: usize = scan.next();
        bottles.push((a, b));
    }
    let mut has = [0usize; 1024];
    let mut has2 = [0usize; 1024];
    for &(a, b) in &bottles {
        has[a] += 1;
        if a == b {
            has2[a] += 1;
        }
    }
    for &(a, b) in &bottles {
        if a != b || has2[a] > 1 {
            has[b] = 0;
        }
        if a == b && has[a] > 1 && has2[a] == 1 {
            has[a] = 1;
        }
    }
    let result: usize = has.iter().sum();
    println!("{}", result);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}

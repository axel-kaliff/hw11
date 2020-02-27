use std::io::{self, BufRead, Write};
//use std::time::{Duration, Instant};

pub fn fenwick() {
    const N: usize = 500000;

    let mut n = 0;

    let mut tree: [i64; 2 * N] = [0; 2 * N];
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for (count, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        let nums: Vec<&str> = line.split_whitespace().collect();

        if count == 0 {
            n = nums[0].parse().unwrap();
            for i in (0..n).rev() {
                tree[i as usize] = tree[(i << 1) as usize] + tree[(i << 1 | 1) as usize];
            }
        } else if nums[0] == "+" {
            let mut p: i64 = nums[1].parse().unwrap();
            let value: i64 = nums[2].parse().unwrap();

            //insert
            tree[(p + n) as usize] = value;
            p = p + n;

            while p > 1 {
                tree[(p >> 1) as usize] = tree[p as usize] + tree[(p ^ 1) as usize];

                p >>= 1;
            }
        } else {
            //query
            let mut res = 0;
            let mut l = 0;
            let mut r: i64 = nums[1].parse().unwrap();
            l += n;
            r += n;

            while l < r {
                if (l & 1) == 1 {
                    res += tree[l as usize];
                    l += 1;
                }

                if (r & 1) == 1 {
                    r -= 1;
                    res += tree[r as usize];
                }

                l >>= 1;
                r >>= 1;
            }
            writeln!(handle, "{}", res);
        }
    }
}

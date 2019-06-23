use std::cmp::min;
use std::u32::MAX;

fn main() {
    let x: usize = 2023;
    let mut dp = Dp { table: vec![None; x + 1] };
    let y = dp.dp(&[2,3,4], x as i32);
    println!("{} {}", x, y);
}

struct Dp {
    table: Vec<Option<u32>>
}

impl Dp {
    fn dp(&mut self, coins: &[u32], x: i32) -> u32 {
        self.table[0] = Some(0);

        for k in 1..x + 1 {
            self.table[k as usize] = Some(MAX - 1);

            for c in coins {
                let xx = k - (*c as i32);

                if xx >= 0 {
                    let y = match self.table[xx as usize] {
                        Some(value) => value + 1,
                        None => MAX
                    };

                    self.table[k as usize] = min(self.table[k as usize], Some(y))
                }
            }
        }

        return self.table[x as usize].unwrap();
    }

    fn dp_recurse(&mut self, coins: &[u32], x: i32) -> u32 {
        if x <= 0 { return 0 }

        // x is always allowed to cast to usize because >= 0
        match self.table[x as usize] {
            Some(xx) => return xx,
            _ => ()
        }

        let xx = coins.iter().fold(MAX, |acc, c| {
            return min(acc, self.dp_recurse(coins, x - (*c as i32)) + 1);
        });
        self.table.insert(x as usize, Some(xx));
        return xx;
    }
}

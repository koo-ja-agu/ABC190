use std::vec::Vec;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
             std::io::stdin().read_to_string(&mut s).unwrap();
             s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        N: usize,
        M: usize,
        AB: [(usize1, usize1); M],
        K: usize,
        CD: [(usize1,  usize1); K],
    }
    let mut answer = 0;
    let k2 = 1 << K;
    for s in 0..k2 {
        let mut dish : Vec<i32> = Vec::new();
        for i in 0..N {
            dish.push(0);
        }
        let mut now = 0;
        for i in 0..K {
            if s >> i & 1 == 1 {
                dish[CD[i].0] += 1;
            } else {
                dish[CD[i].1] += 1;
            }
        }
        for i in 0..M {
            if dish[AB[i].0] == 0 {
                continue;
            }
            if dish[AB[i].1] == 0 {
                continue;
            }
            now += 1;
        }
        answer = if now > answer {
            now
        } else {
            answer
        };
    }
    println!("{}",answer);
}
//{"name":"B. Зеркало в строке","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"http://codeforces.com/contest/1616/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10\ncodeforces\n9\ncbacbacba\n3\naaa\n4\nbbaa\n","output":"cc\ncbaabc\naa\nbb\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZerkaloVStroke"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod9;
use algo_lib::misc::rand::Random;
use algo_lib::strings::hash_string::{default_hash_string_context, Context};
use algo_lib::{dbg, out, out_line};
use std::cmp::Ordering;

type Mod = Mod9;

fn solve_task(s: &Vec<u8>) -> String {
    let len = s.len();
    let ctx = default_hash_string_context::<Mod>(s.len());
    let s = ctx.create_from_string(&s);

    let build = |prefix_len: usize| {
        let prefix = ctx.substring(&s, 0..prefix_len);
        ctx.concat(&prefix, &ctx.rev(&prefix))
    };

    let mut best = build(1);
    for len in 2..=len {
        let check = build(len);
        if ctx.compare(&check, &best) == Ordering::Less {
            best = check;
        }
    }

    "hhh".into()

    //String::from_utf8(ctx.to_string(&best)).unwrap()
}

fn solve(input: &mut Input, _test_case: usize) {
    let _len = input.usize();
    let s = input.string_as_vec();

    let result = solve_task(&s);

    out_line!(result);
}

fn stress() -> bool {
    let n = 100_000;
    let mut s = vec![b'a'; n];
    // let mut rnd = Random::new(787788);
    // for i in 0..n {
    //     s[i] = (b'a' + rnd.next_in_range(0, 26) as u8);
    // }
    s[0] = b'b';
    solve_task(&s);
    true
}

pub(crate) fn run(mut input: Input) -> bool {
    if stress() {
        return true;
    }
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN

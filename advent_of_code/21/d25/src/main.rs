use d25::calculate;
use std::fs;

fn main() {
    /*
        let init_map = "v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>";
        let res = "..>>v>vv..
    ..v.>>vv..
    ..>>v>>vv.
    ..>>>>>vv.
    v......>vv
    v>v....>>v
    vvv.....>>
    >vv......>
    .>v.vv.v..";
        assert_eq!(calculate(init_map), res);
        */
    use std::time::Instant;
    let real_task = fs::read_to_string("real_input.txt").unwrap();
    let now = Instant::now();

    // Code block to measure.
    {
        calculate(real_task.as_str());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

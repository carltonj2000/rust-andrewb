use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    println!("String:\n{contents}");

    let lines: Vec<&str> = contents.split("\n").collect();
    println!("Lines:\n{:?}", lines);

    let mut _idx: usize = 0;
    let mut tot: usize = 0;
    let mut tot1: usize = 0;
    let mut tot2: usize = 0;
    let mut tot3: usize = 0;
    for ele in lines {
        if ele == "" {
            if tot > tot1 {
                tot3 = tot2;
                tot2 = tot1;
                tot1 = tot;
            } else if tot > tot2 {
                tot3 = tot2;
                tot2 = tot;
            } else if tot > tot3 {
                tot3 = tot;
            }
            _idx += 1;
            tot = 0;
        } else {
            tot += ele.parse::<usize>().unwrap();
        }
    }

    tot = tot1 + tot2 + tot3;
    println!("The 3 max calories add up to {}.", tot);
}

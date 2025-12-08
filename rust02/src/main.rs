use std::fs;

fn main() {
    // let content = fs::read_to_string("input/test").unwrap();
    let content = fs::read_to_string("input/input").unwrap();
    let mut res:u64 = 0;

    for line in content.split(','){
        let v: Vec<&str> = line.split('-').collect();
        let mut i: u64= v[0].parse().unwrap();
        // let hehe = v[1];
        // println!("{}", v[0]);
        // println!("{hehe}= v[1]");
        let end :u64= v[1].trim_end_matches("\n").parse().unwrap();

        while i <= end {
            let log = i.ilog10();
            if log%2 != 0{
                let smaller = i / 10u64.pow(log / 2 + 1);
                let bigger = i - smaller;
                // println!("smaller {smaller} bigger {bigger}");
                if smaller * 10u64.pow(log/2 + 1) == bigger{
                    println!("correct = {i}");
                    res += i; 
                }
                // println!("{i}");
            }
            i += 1;
        }
    }
    println!("{res}");
}

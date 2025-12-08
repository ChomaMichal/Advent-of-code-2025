use std::fs;

fn main() {
    // let content = fs::read_to_string("input/test").unwrap();
    let content = fs::read_to_string("input/input").unwrap();
    let mut res:u64 = 0;

    for line in content.split(','){
        let v: Vec<&str> = line.split('-').collect();
        let mut i: u64= v[0].parse().unwrap();
        let end :u64= v[1].trim_end_matches("\n").parse().unwrap();

        'outer: while i <= end {
            let s = i.to_string();
            'l: for j in 1..=s.len() / 2{
                for chunk in  s.as_bytes().chunks(j).collect::<Vec<&[u8]>>().windows(2){
                    if chunk[0]!=chunk[1]{
                        continue 'l;
                    }
                }
                res += i; 
                break ;
            }
            i += 1;
        }
    }
    println!("{res}");
}

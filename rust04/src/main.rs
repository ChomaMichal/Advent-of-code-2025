use std::fs;


fn main() {
    // let input = fs::read_to_string("input/test").unwrap();
    let input = fs::read_to_string("input/input").unwrap();
    let mut lines:Vec<String> = input.split("\n").map(|s| s.to_owned()).collect();
    let mut res = 0;
    let mut old = 1;

    while old != res{
        old = res;
        for i in 0..lines.len(){
            // println!("i = {i}");
            for j in 0..lines[i].len(){
                // println!("J = {j}");
                let mut amount = 0;
                if lines[i].chars().nth(j).unwrap() != '@'{continue}

                if i != 0 && j != 0{
                    // println!("i != 0 && j != 0");
                    if lines[i - 1].chars().nth(j - 1).unwrap() == '@'{amount += 1}
                }
                if i != 0{
                    // println!("i != 0");
                    if lines[i - 1].chars().nth(j).unwrap() == '@'{amount += 1}
                }
                if j != 0{
                    // println!("j != 0");
                    if lines[i].chars().nth(j - 1).unwrap() == '@'{amount += 1}
                }
                if i != lines.len() - 2  && j != lines[i].len() - 1{
                    // println!("i != len && j != len");
                    if lines[i + 1].chars().nth(j + 1).unwrap() == '@'{amount += 1}
                }
                if i != lines.len()- 2 {
                    // println!("i != len");
                    if lines[i + 1].chars().nth(j).unwrap() == '@'{amount += 1}
                }
                if j != lines[i].len() - 1{
                    // println!("j != len");
                    if lines[i].chars().nth(j + 1).unwrap() == '@'{amount += 1}
                }
                if i !=  0 && j != lines[i].len() - 1{
                    // println!("i != 0 && j != len");
                    if lines[i - 1].chars().nth(j + 1).unwrap() == '@'{amount += 1}
                }
                if i != lines.len() - 2 && j != 0{
                    // println!("i != len  && j != 0");
                    if lines[i + 1].chars().nth(j - 1).unwrap() == '@'{amount += 1}
                }
                if amount  < 4{
                    // println!("{res} == res");
                    res +=1;
                    lines[i] = format!("{}.{}", &lines[i][0..j], &lines[i][j + 1..]).to_owned(); 
                }
            }
        }
    }
    println!("{res}");

}


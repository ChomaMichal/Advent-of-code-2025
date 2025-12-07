use std::fs;

fn main(){
    let input = fs::read_to_string("input/input").unwrap();
    // let input = fs::read_to_string("input/test").unwrap();
    let s :Vec<Vec<char>> = input
        .lines()
        .map(|a| a.chars().collect())
        .collect();
    let mut res: u64 = 0;
    let mut flag: u64;
    let mut oper :char = 'a';
    let mut arr: Vec<u64> = Vec::new();

    for i in 0..(s[0].len()){
        let mut num : u64= 0;
        flag = 0;
        for j in 0..(s.len()){
            if s[j][i] == '*'{
                oper = '*';
                continue;
            }
            if s[j][i] == '+'{
                oper = '+';
                continue;
            }
            if s[j][i] == ' '{
                flag += 1;
                continue;
            }
            if s[j][i] == '\n' {continue}
            // println!("{}", s[j][i]);
            num = num *10 + s[j][i].to_digit(10).unwrap() as u64;
        }
        // println!("{num} == num");
        // println!("{flag} == flag");
        if flag as usize == s.len(){
            println!("hehe");
            if oper == '*'{
                let mut tmp :u64 = 1;
                for i in 0..arr.len(){
                    tmp *= arr[i];
                }
                    res += tmp;
            }
            if oper == '+'{
                for i in 0..arr.len(){
                    res+= arr[i];
                }
            }
            arr = Vec::new();
            println!("{res}");
        }
        else{
            arr.push(num);
        }
    }
    if oper == '*'{
        let mut tmp :u64 = 1;
        for i in 0..arr.len(){
            tmp *= arr[i];
        }
            res += tmp;
    }
    if oper == '+'{
        for i in 0..arr.len(){
            res+= arr[i];
            }
    }
    // for line in s{
        // 3281418
        // 3263827
    //     for word in  line{
    //         print!("|{word}|");
    //     }
    //     println!("\n------");
    // }
    println!("{res}");
}

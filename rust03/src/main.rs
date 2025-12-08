use std::fs;

fn main() {
    let input = fs::read_to_string("input/input").unwrap();
    let mut i: i32= 0;
    for line in input.split('\n'){

        let mut first: i32= 0;
        for l in line.chars().rev().skip(1){
            let num :i32= l.to_string().parse().unwrap();
            if num > first{
                first = num;
            }
        }

        let mut flag :i32 = 0;
        let mut second :i32 = 0;
        for l in line.chars(){
            let num :i32= l.to_string().parse().unwrap();
            if num == first && flag == 0{
                flag = 1;
            }
            else if flag == 1{
                if num > second{
                    second = num;
                }
            }
        }

        println!("{line}");
        println!("{}", first * 10 + second);
        // assert_ne!(first, 0);
        // assert_ne!(second, 0);
        i += first * 10 + second;
    }
    println!("{i}");
}

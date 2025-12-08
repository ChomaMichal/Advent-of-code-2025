use std::fs;


fn main() {
    let content = fs::read_to_string("input/input").unwrap();
    let mut num :i32 = 50;
    let mut res :i32 = 0;

    for line in content.lines(){
        if line.starts_with('L'){
            let tmp = &line[1..];
            let mut hehe:i32= tmp.parse().unwrap();
            while hehe != 0{
                num -=1;
                if num == -1{
                    num = 99
                }
                if num == 0{
                    res += 1;
                }
                hehe -= 1;
            }
        } else {
            let tmp = &line[1..];
            let mut hehe:i32 = tmp.parse().unwrap();
            while hehe != 0{
                num +=1;
                if num == 100{
                    num = 0;
                }
                if num == 0{
                    res += 1;
                }
                hehe -= 1;
            }
        }
    }
    println!("{res}");
    
}

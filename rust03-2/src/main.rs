use std::fs;

// struct Hello{
//     num_digits :usize,
//     index :usize,
// }

//
// fn search(line: &str) {
//
//     let mut found_digits: Vec<usize>;
//     found_digits.push(0);
//     for digits_found in 0..2 {
// let curr = line.chars().skip(*(found_digits.last().unwrap())).take(line.len() - idk.num_value - digits_found + 1).map(|c| c.to_string().parse().unwrap()).max();
//         let curr = curr.unwrap();
//         println!("cur {}", curr);
//         panic!();
//     }
//
//
//     // for i in idk.index as usize..(line.len() - idk.num.ilog10() as usize) {
//     //     let num : usize = 
//     // }
//
//    idk
// }

fn main() {
    let input = fs::read_to_string("input/input").unwrap();
    // let input = fs::read_to_string("input/test").unwrap();
    let mut res:usize = 0;
    for line in input.split('\n'){
        let mut fin :usize= 0;
        let mut last: usize = 0;
        // let mut flag: usize = 0;
        let mut index = 0;

        if line.len() == 0{continue;}
        for j in 0..12{
            // println!("{j}");
            for i in index..(line.len() - 11 + j){
                let c = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                // println!("comparing {c} {last}");
                if c as usize> last{
                    last  = c as usize;
                    index = i + 1;
                    println!("{last} hehe");
                }
            }
            fin = (fin * 10) + last; 
            last = 0;
            // flag = 0;1
            // println!("j == {j}");
            println!("{fin}");
        }
        res += fin;
    }
    println!("{res}");
}

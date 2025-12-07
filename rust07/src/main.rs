use std::fs;


fn rec(
    lines :&Vec<Vec<char>>,
    i :usize,
    j :usize,
    max:usize,
    memory: &mut Vec<Vec<Option<usize>>>
    )-> usize{
    let mut res :usize = 0;

        // println!("{}, lines[{}][{}]", lines[i][j], i, j);
    if i == lines.len() -1 {return 0}
    if let Some(v) = memory[i][j]{
        return v;
    }
    // println!("{}",j);
    if lines[i][j] == '^'{
        res += 1;
        // println!("{}, lines[{}][{}]", lines[i][j], i, j);
        if j != 0{
            res += rec(lines, i + 1, j -1, max, memory);
        }
        if j != max - 1{
            res += rec(lines, i + 1, j + 1, max, memory);
        }
    }else {
        // println!("{}, lines[{}][{}]", lines[i][j], i, j);
        res += rec(lines, i + 1, j, max,memory);
    }
    memory[i][j] = Some(res);
    return res;
}

fn main() {
    let input: String = fs::read_to_string("input/input").unwrap();
    // let input = fs::read_to_string("input/test").unwrap();
    let lines: Vec<Vec<char>> = input.lines()
        .map(|e| e.chars().collect())
        .collect();
    // let mut res = 0;
    // let mut splits =  0;
    // let mut res1 = 0;
    // let mut res2 = 0;
    //
    //
    //
    //
    //
    //
    // for i in 0..(lines.len()){
    //     if i == lines.len() - 1 {break}
    //
    //     for j in 0..(lines[i].len()){
    //         if lines[i][j] == 'S'{
    //             if lines[i + 1][j] == '^'{
    //                 splits += 1;
    //                 if j != 0 {
    //                     if j != 0{
    //                         lines[i + 1][j - 1] = 'S';
    //                         res2 += 1;
    //                     }
    //                     if j != lines[0].len() - 1{
    //                         lines[i + 1][j + 1] = 'S';
    //                         res1 += 1;
    //                     }
    //
    //                     // if j != 0 && lines[i + 1][j - 1] != 'S'{
    //                     //     res += 1;
    //                     //     lines[i + 1][j - 1] = 'S'
    //                     // }
    //                     // if j != lines[i].len() && lines[i + 1][j + 1] != 'S'{
    //                     //     res += 1;
    //                     //     lines[i + 1][j + 1] = 'S'
    //                     // }
    //                 }
    //             } else {
    //                 lines[i + 1][j] = 'S';
    //             }
    //         }
    //     }
    // }
    // for line in lines.iter(){
    //     for c in line.iter(){
    //         print!("{c}");
    //     }
    //     println!("");
    // }
    

    let max = lines[0].len();
    let mut memory: Vec<Vec<Option<usize>>> = vec![vec![None; max]; lines.len()];
    for (j, &c) in lines[0].iter().enumerate(){
        if c == 'S'{
            println!("{}",rec(&lines, 0, j, max, &mut memory));
        }
    }

    // println!("{res1} == res 1");
    // println!("{res2} == res 2");
    // println!("{splits} == splits");
    // res = res1 + res2;
    // println!("{res}");
}

use std::fs;

#[derive(Copy,Clone, Debug, PartialEq, Eq)]
struct Idk{
    i:(usize,usize),
    j:(usize,usize),
    size:usize,
}

#[derive(Copy,Clone, Debug, PartialEq, Eq)]
enum Square{
    BLACK,
    RED,
    GREEN,
    POISON,
}

fn get_order_of_biggest(arr: Vec<(usize, usize)>) -> Vec<Idk>{
    let mut res: Vec<Idk> = vec![];
    for (i, s) in arr.iter().enumerate(){
        for  p in arr.iter().skip(i + 1){
            let tmp = (1 + s.0.abs_diff(p.0)) * (1 + s.1.abs_diff(p.1));
            let hehe :Idk = Idk{i:s.clone(), j:p.clone(), size:tmp};
            res.push(hehe);
        }
    }
    res.sort_by_key(|a| a.size);
    return res
}


fn print_grid(grid: &Vec<Vec<Square>>){
    for row in grid{
        for item in row{
            match item{
                Square::BLACK => {print!(".");}
                Square::RED => {print!("R");}
                Square::GREEN => {print!("G");}
                Square::POISON => {print!("P")}
            } 
        }
        println!("");
    }
}

fn fill_in_line(grid: &mut Vec<Vec<Square>>, x1:usize, y1:usize, x2:usize, y2:usize){
    if x1 == x2{
        for i in y1+1..y2{
            grid[i][x1] = Square::GREEN;
        }
    }
    if y1 == y2{
        for i in x1+1..x2{
            grid[y1][i] = Square::GREEN;
        }
    }
}

fn iterative_filling(grid: &mut Vec<Vec<Square>>) {
    for row in grid.iter_mut(){
        let mut flag = 0;
        for piece in row.iter_mut(){
            if flag == 0 && (*piece == Square::GREEN || *piece == Square::RED) {flag = 1}
            else if flag == 1 && (*piece == Square::GREEN || *piece == Square::RED) {flag = 0}
            else if flag == 0 {*piece = Square::POISON}
        }

    }

}

fn flood_fill(grid: &mut Vec<Vec<Square>>, x: usize, y: usize) {
    if grid.is_empty() || x >= grid.len() { return; }
    if y >= grid[x].len() { return; }

    match grid[x][y] {
        Square::POISON | Square::RED | Square::GREEN => return,
        _ => {}
    }
    grid[x][y] = Square::POISON;
    if x > 0 {
        flood_fill(grid, x - 1, y);
    }
    if y > 0 {
        flood_fill(grid, x, y - 1);
    }
    if x + 1 < grid.len() {
        flood_fill(grid, x + 1, y);
    }
    if y + 1 < grid[x].len() {
        flood_fill(grid, x, y + 1);
    }
}

fn fill_in_grid(grid: &mut Vec<Vec<Square>>, arr:Vec<(usize,usize)>){
    for i in 0..arr.len(){
        if i == arr.len()-1 {break;}
        if arr[i].0 == arr[i+1].0 {fill_in_line(grid, arr[i].0, arr[i].1, arr[i+1].0, arr[i+1].1)}
        for j in i+1..arr.len(){
            if arr[j].1 == arr[i].1 {fill_in_line(grid, arr[i].0, arr[i].1, arr[j].0, arr[j].1)}
        }
    }
}

fn flood_fill_dfs_iter(
    grid: &mut [Vec<Square>],
    start_x: usize,
    start_y: usize,
    target: Square,
    replacement: Square,
) {
    if target == replacement { return; }
    if grid.is_empty() { return; }
    if start_x >= grid.len() { return; }
    if start_y >= grid[start_x].len() { return; }

    // quick check: nothing to do if start isn't the target
    if grid[start_x][start_y] != target { return; }

    let mut stack = Vec::new();
    stack.push((start_x, start_y));

    while let Some((x, y)) = stack.pop() {
        // bounds and current value checks
        if x >= grid.len() { continue; }
        if y >= grid[x].len() { continue; }
        if grid[x][y] != target { continue; }

        // mark / replace
        grid[x][y] = replacement;

        // push neighbors (check row-lengths for different-sized rows)
        if x > 0 && y < grid[x - 1].len() { stack.push((x - 1, y)); }
        if x + 1 < grid.len() && y < grid[x + 1].len() { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < grid[x].len() { stack.push((x, y + 1)); }
    }
}

fn steps_on_poison(grid: &Vec<Vec<Square>>, hehe:Idk) -> i32{
        // println!("{:?}", hehe);
    let a = hehe.i.0;
    let b = hehe.j.0;
    for i in a.min(b)..b.max(a){
        let c  = hehe.i.1;
        let d  = hehe.j.1;
        for j in c.min(d)..d.max(c){
            // println!("hehe");
            if grid[j][i] == Square::POISON {return -1}
        }
    }
    return 1;
}


fn main() {
    let input = fs::read_to_string("input/input").unwrap();
    // let input = fs::read_to_string("input/test").unwrap();

    let lines = input.lines();

    let mut arr : Vec<(usize, usize)> = vec![];

    for line in lines.into_iter(){
        let hehe: Vec<&str> = line.split(',').map(|e| e.trim()).collect();
        if line.is_empty() { break;}
        let tmp :(usize, usize) = (
            hehe[0].parse().unwrap(),
            hehe[1].parse().unwrap(),
        );
        arr.push(tmp);
        // println!("{:?}", tmp);
    }
    arr.sort_by(|a,b| a.1.cmp(&b.1));
    let y_max = arr.last().unwrap().1;
    arr.sort_by(|a,b| a.cmp(b));
    let x_max = arr.last().unwrap().0;
    // println!("{y_max} == y_max");
    // println!("{x_max} == x_max");
    let mut grid: Vec<Vec<Square>> = vec![vec![Square::BLACK; x_max + 1]; y_max + 1];
    for item in arr.clone(){
        // println!("{:?}", item);
        grid[item.1][item.0] = Square::RED;
        // print_grid(grid.clone());
    }

    fill_in_grid(&mut grid, arr.clone());
    flood_fill_dfs_iter(&mut grid, 0,0, Square::BLACK, Square::POISON);
    // iterative_filling(&mut grid);
    // flood_fill(&mut grid, 0, 0);
    print_grid(&grid); 

    let s_arr = get_order_of_biggest(arr);

    for item in s_arr.iter().rev(){
        if steps_on_poison(&grid, item.clone()) == 1 {
            println!("{}", item.size);
            return;
        }
    }

    return ;


}




//
// fn main() {
//     let input = fs::read_to_string("input/input").unwrap();
//     // let input = fs::read_to_string("input/test").unwrap();
//
//     let lines = input.lines();
//
//     let mut arr : Vec<(usize, usize)> = vec![];
//
//     for line in lines.into_iter(){
//         let hehe: Vec<&str> = line.split(',').map(|e| e.trim()).collect();
//         if line.is_empty() { break;}
//         let tmp :(usize, usize) = (
//             hehe[0].parse().unwrap(),
//             hehe[1].parse().unwrap(),
//         );
//         arr.push(tmp);
//         // println!("{:?}", tmp);
//     }
//     arr.sort_by(|a,b| a.cmp(b));
//     println!("{:?}", arr.last().unwrap());
//     let mut res: usize = 0;
//     for (i, s) in arr.iter().enumerate(){
//         for  p in arr.iter().skip(i + 1){
//             let tmp = (1 + s.0.abs_diff(p.0)) * (1 + s.1.abs_diff(p.1));
//             // println!("{} = s.0|{} = s.1|{} = p.0|{} = p.1|{tmp} = tmp", s.0, s.1,p.0,p.1);
//             if tmp > res {
//                 res = tmp;
//             }
//         }
//     }
//     print!("{res}");
//
// }

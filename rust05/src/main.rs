use std::fs;
// fn main() {
//     let input = fs::read_to_string("input/input").unwrap();
//     // let input = fs::read_to_string("input/test").unwrap();
//     let mut flag = 0;
//     let mut res= 0;
//     let mut items: Vec<u64> = vec![];
//     let mut ranges: Vec<(u64, u64)> = vec![];
//
//     for line in input.split("\n"){
//         if line.is_empty(){
//             flag = 1;
//             continue;
//         }
//         if flag == 0{
//             let tmp : Vec<&str> = line.split("-").collect();
//             ranges.push((tmp[0].parse().unwrap(), tmp[1].parse().unwrap()));
//         }
//         else{
//             items.push(line.parse::<u64>().unwrap());
//         }
//     }
//     for  item in items{
//         'inner: for range in &ranges{
//             if item >= range.0 && item <= range.1{
//                 res += 1;
//                 // println!("{item} == item");
//                 break 'inner;
//             }
//         }
//     }
//     println!("{res}");
//
// }

fn main() {
    let input = fs::read_to_string("input/input").unwrap();
    // let input = fs::read_to_string("input/test").unwrap();
    let mut items: Vec<(u64, u64)> = vec![];
    let mut ranges: Vec<(u64, u64)> = vec![];

    for line in input.split("\n"){
        if line.is_empty(){
            break;
        }
        let tmp : Vec<&str> = line.split("-").collect();
        ranges.push((tmp[0].parse().unwrap(), tmp[1].parse().unwrap()));
    }
    let mut flag;
    let mut i =0;
    // println!("{}", ranges.len());
    for range in ranges{
        if items.is_empty(){ items.push(range); continue;}
        flag = 0;
        'items: for i in 0..items.len(){
            // println!("{}-{}",items[i].0, items[i].1);
            if range.0 > items[i].0 && range.0 < items[i].1{
                if range.1 < items[i].1{
                    flag = 1;
                    break 'items;
                }
                if range.1 > items[i].1{
                    flag = 1;
                    items[i].1 = range.1;
                    break 'items;
                }
            }
            if range.1 > items[i].0 && range.1 < items[i].1{
                if range.0 > items[i].0{
                    flag = 1;
                    break 'items;
                }
                if range.0 < items[i].1{
                    flag = 1;
                    items[i].0 = range.0;
                    break 'items;
                }
            }
        }
        if flag == 0{
            items.push(range);
        }
        i += 1;
        // println!("{}", i);
        // println!("{} = items len", items.len());
        // println!("{} / {}", ranges.iter().position(|&x| x == range.to_owned()).unwrap(), ranges.len());
    }
    let mut res = 0;
    for i in 0..items.len(){
        // println!("{}-{}",items[i].0, items[i].1);
        for j in 0..items.len(){
            if items[i].1 >= items[j].0 && i != j && items[i].1 <= items[j].1  {
                if items[i].0 >= items[j].0 && i != j && items[i].0 <= items[j].1  {
                    items[i].0 = 0;
                    items[i].1 = 0;
                    break;
                }
                println!("{}-{} in if i ",items[i].0, items[i].1);
                println!("{}-{} in if j",items[j].0, items[j].1);
                items[i].1 = items[j].0 - 1;
                println!("{}-{} after",items[i].0, items[i].1);
                assert!(items[i].1 >= items[i].0);
            }
        }
    }
    
    for item in items{
        if item.0 == 0 && item.1 == 0{continue;}
        println!("{}-{}",item.0, item.1);
        res += item.1 - item.0 + 1;
    }
    println!("{res}"); 
}

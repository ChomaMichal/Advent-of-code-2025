use std::fs;
fn  e_dis(p: &(f64,f64,f64), q: &(f64,f64,f64))->f64{
    let ret:f64;
    ret = ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2) +(p.2 - q.2).powi(2)).sqrt();
    return ret;
}

fn main() {
    // let input = fs::read_to_string("input/input").unwrap();
    let input = fs::read_to_string("input/test").unwrap();
    let lines = input.lines();
    println!("{}=lines.len()", lines.clone().collect::<Vec<&str>>().len());
    let mut coor :Vec<(f64, f64, f64)> = vec![];
    // 0 is distance 1 is position of first 2 is position of second
    let mut dis : Vec<(f64, usize, usize)> = vec![];
    println!("{}=lines[19]", lines.to_owned().collect::<Vec<&str>>()[19]);
 
    for line in lines.clone().into_iter(){
        let hehe: Vec<&str> = line.split(',').map(|e| e.trim()).collect();
        let triple :(f64, f64, f64) =(
            hehe[0].parse().unwrap(),
            hehe[1].parse().unwrap(),
            hehe[2].parse().unwrap(),
        );
        coor.push(triple);
    }

    // now the are parsed to numbers
    //

    for (i,p) in coor.iter().enumerate(){
        if i == coor.len() - 1 {break}
        for (j,q) in coor.iter().enumerate().skip(i + 1){
            dis.push((
                e_dis(p, q),
                i,
                j,
            ))
        }
    }
    // println!("{}==dis.len()", dis.len());
    dis.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
    // println!("{}==dis.len()", dis.len());
    // for (i, s) in dis.iter().enumerate() {
    //     println!("{}| {},{},{}",i, s.0,s.1,s.2);
    // }


    // println!("{}==dis.len()", dis.len());
    let mut conected :Vec<Vec<usize>> = vec![];
    for i in 0..lines.into_iter().count() {
        conected.push(vec![i]);
    }

    let mut connections_left = 9;
    for s in dis {
        if connections_left == 0 {break}
        if conected.iter().position(|x| x.contains(&s.1)) == conected.iter().position(|x| x.contains(&s.2)) {
            continue;
        }
        let c1_index = conected.iter().position(|x| x.contains(&s.1)).unwrap();
        let mut cir = conected.remove(c1_index);
        conected.iter_mut().find(|x| x.contains(&s.2)).unwrap().append(&mut cir);
        connections_left -= 1;
    }
    let mut res = 1;

    conected.sort_by(|b, a| a.len().partial_cmp(&(b.len())).unwrap());

    for e in conected.iter() {
        println!("{:?}", e);
        res *= e.len();
    }
    println!("a{res}");
}

fn testing(x: String) {

}

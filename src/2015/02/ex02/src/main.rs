use std::io::{self, Read};
use std::fs::File;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("--- Day 2: I Was Told There Would Be No Math ---");
    println!("--- Part 2 ---");
    let mut total_surface_area = 0;
    let mut total_ribbon = 0;
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let dimensions: Vec<i32> = line.split("x").map(|item| {
            item.trim().parse().unwrap()
        }).collect();
        let [l, w, h] = dimensions.as_slice() else { todo!() };
        println!("{l} {w} {h}");
        let surface_area_l_w = l * w;
        let surface_area_l_h = l * h;
        let surface_area_w_h = w * h;
        let areas = Vec::from([surface_area_l_w, surface_area_l_h, surface_area_w_h]);
        let surface_area_slack = areas.iter().min().unwrap();
        let surface_area = 2 * (surface_area_l_w + surface_area_l_h + surface_area_w_h) + surface_area_slack;
        total_surface_area += surface_area;
        println!("lw: {surface_area_l_w} lh: {surface_area_l_h} wh: {surface_area_w_h} slack: {surface_area_slack} item surface area: {surface_area}");

        // ribbon
        let mut lengths = [l, w, h];
        lengths.sort_by(|a, b| a.cmp(b)); // sort so we can get the two minimum lengths
        let ribbon_present_length = 2 * (lengths[0] + lengths[1]);
        let ribbon_bow_length = l * w * h;
        let ribbon_length = ribbon_present_length + ribbon_bow_length;
        total_ribbon += ribbon_length;
        println!("ribbon present length: {ribbon_present_length} ribbon bow length: {ribbon_bow_length} total length: {ribbon_length}");
    });

    println!("Total surface area needed: {total_surface_area} square feet"); // 1606483
    println!("Total ribbon length needed: {total_ribbon} feet"); // 3842356
}
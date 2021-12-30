use aocutils::*;

const WINDOW_SIZE: usize = 3;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = get_aoc_input("01a");
    let mut depths = input.lines();
    let mut last_depth = to_i32(depths.next().unwrap());
    let mut depth_increases = 0;
    for depth in depths {
        let depth_i32 = to_i32(depth);
        if last_depth < depth_i32 {
            depth_increases += 1;
        }
        last_depth = depth_i32;
    }
    println!("part1: {}", depth_increases);
}

fn part2() {
    let depthvec: Vec<i32> = get_aoc_input("01b").lines().map(|s| to_i32(&s)).collect();
    let mut last_depth_sum = depthvec[0] + depthvec[1] + depthvec[2];
    if depthvec.len() < WINDOW_SIZE {
        println!("part2: {}", 0);
        return;
    }

    let mut depth_increases = 0;
    for i in 1..depthvec.len()-(WINDOW_SIZE-1){ 
        let d0 = depthvec[i];
        let d1 = depthvec[i+1];
        let d2 = depthvec[i+2];
        let sum = d0 + d1 + d2;
        if last_depth_sum < sum {
            depth_increases += 1;
        }
        last_depth_sum = sum;
    }
    println!("part2: {}", depth_increases);
}


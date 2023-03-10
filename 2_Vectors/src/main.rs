fn main() {
    let numbers = vec![10, 20, 30, 40, 50, 60];

    println!("Iterate over vector:");
    for x in &numbers {
        println!("Number: {}", x);
    }

    println!("Slice from 0 to 2");
    let slice = &numbers[0..2];
    for x in slice {
        println!("Number: {}", x);
    }

    let mut subslices: Vec<Vec<i32>> = vec![];
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - i {
            if i <= j {
                subslices.push(numbers[i..=j].to_vec());
            }
        }
    }

    println!("Continuous Subslices:");
    for x in &subslices {
        println!("Number: {:?}", x);
    }
}

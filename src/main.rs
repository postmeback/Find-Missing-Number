fn find_missing_number(arr: Vec<i32>) -> i32 {
    let n = arr.len() as i32 + 1;
    let expected_sum: i32 = (n * (n+1)) / 2;

    let actual_sum: i32= arr.iter().sum();

    let missing_number = expected_sum - actual_sum;

    return  missing_number;
}

fn main(){
    let arr = vec![1,2,4,5,6,7];

    let missing_number = find_missing_number(arr);

    println!("The number is: {}", missing_number);
}
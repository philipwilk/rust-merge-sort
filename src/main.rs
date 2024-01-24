use std::vec;
use tokio::task::JoinSet;

fn join(mut left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    for (i, x) in left.iter().enumerate() {
        if x < &right[i] {
            left.insert(i, right[i]);
        }
    }
    left
}

async fn mergeSort(mut input: Vec<i32>) -> Vec<i32> {
    if input.len() > 1 {
        let right: Vec<i32> = input.split_off(input.len()+1 /2);
        let left: Vec<i32> = input;
        let mut taskSet = JoinSet::new();
        taskSet.spawn(mergeSort(left));
        taskSet.spawn(mergeSort(right));

        let mut sorteds: Vec<Vec<i32>> = vec![];
        while let Some(res) = taskSet.join_next().await {
            sorteds.push(res.unwrap());
        }
        join(sorteds[0], sorteds[1])
    }
    else {
        input
    }
}

#[tokio::main]
async fn main() {
    let start: Vec<i32> = vec![ 1, 6, 10, 2, 45, 23, 94, 67, 5, 9, 50 ];
    let end = mergeSort(start.clone()).await;
    println!("{:?} sorted -> {:?}", start, end);
}


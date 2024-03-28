// bubble sort for numbers (i32)
fn bubble_sort_i32(arr: &mut Vec<i32>){
    let mut n = arr.len();
    let mut swapped=true;

    while swapped{
        swapped = false;
        for i in 1..n{
            if arr[i-1] > arr[i]{
                arr.swap(i-1,i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn main(){
    let mut numbers = vec![3, 9, 5, 1, 11];
    bubble_sort_i32(&mut numbers);
    println!("{:?}", numbers);
}
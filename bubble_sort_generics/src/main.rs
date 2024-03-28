fn bubble_sort<T:PartialOrd + Copy>(arr: &mut Vec<T>){
    let mut n = arr.len();
    let mut swapped = true;

    while swapped{
        swapped = false;
        for i in 1..n{
            if arr[i-1]>arr[i]{
                arr.swap(i-1, i);
                swapped = true;
            }
        }
        n-=1;
    }
}

fn main(){
    let mut numbers = vec![3, 9, 5, 1, 11];
    let mut words = vec!["hamburger", "pasta", "soda", "chips"];

    bubble_sort(&mut numbers);
    bubble_sort(&mut words);

    println!("Sorted numbers: {:?}", numbers);
    println!("Sorted words: {:?}", words);
}

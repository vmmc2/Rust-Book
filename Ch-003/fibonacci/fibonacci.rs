fn fib(index: i64) -> i64{
    if index == 1 || index == 2{
        return 1;
    }else{
        return fib(index - 1) + fib(index - 2);
    }
}

fn main(){
    for number in 1..10{
        let result: i64 = fib(number);
        println!("The value of the number {number} in the Fibonacci sequence is: {result}.");
    }

    return;
}
fn calculate_rem(total: u32, elapsed:u32) -> u32{
    total - elapsed 
    // no ";" because its an expression it becomes the return value of the function
    // if you add ":" it becomes a statement
}

fn main(){
    let rem = calculate_rem(240, 90);
    println!("remaining: {}s", rem);
}
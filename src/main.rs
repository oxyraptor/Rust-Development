fn calculate_rem(total: u32, elapsed:u32) -> u32{
    total - elapsed
}

fn main(){
    let rem = calculate_rem(240, 90);
    println!("remaining: {}s", rem);
}
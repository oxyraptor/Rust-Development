fn show_song(title: &str, artist:&str){
    println!("Now playing:{}", title);
    println!("Artist:{}", artist);
}

fn main(){
    show_song("Desi Girl", "shararat");
    show_song("shankar-ehesaan-loy", "someone-idk");
}
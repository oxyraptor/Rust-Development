# Termix

Termix is a Rust-based music player project under development.

## Recent Learning

The project now demonstrates Rust variable bindings:

- Variables are immutable by default.
- Use `mut` when a variable needs to be changed after it is created.
- Immutable and mutable bindings help make changes to program state explicit.

- Signed and unsigned variables 
    why do we use signed and unsigned variables?
    -> because, to avoid any bugs during bitwise operation 
    signed i8 have range from -128 to 127
    unsigned u8 have range from 0 to 255

    integers type
    i8          u8
    i16         u16
    i32         u32
    i64         u64
    i128
    
    floating point types
    f32
    f64

    boolean type
    true and false

    character and string
    char stors unicode character like A, B sigle character

    string :
        &str borrows or referes existing text
        String owns its text 
        these are important for memory management

    compound type
    group multiple values
'''
    tuple
    let song_info =" ("Desi Girl","SEL", 4.34);

    it can be acessed as :
        println!("{}", song_info.0);
        println!("{}", song_info.1);
        println!("{}", song_info.2);

    Array
    let playlist = ["song_1", "song_2", "song_3"];

    println!("{}", playlist[0]);

Function:
    fn main(){
        println!("Hello world!");
    }

    #######
    fn song_info(){
        println!("this is the song info");
    }
    fn main(){
        song_info();
    }

    ########
    fn show_song(title)

Memory
'''

finally listen to o rangrez thrugh rust code

using : cargo run --release -- "Songs/O Rangrez.mp3"

had to cross complied the code for windows as i was using wsl and made the program run natively on windows from wsl

commands for this are
sudo apt update
sudo apt install -y mingw-w64

rustup target add x86_64-pc-windows-gnu


./target/x86_64-pc-windows-gnu/release/termix.exe "$(wslpath -w 'Songs/O Rangrez.mp3')"


i had to do this because i was getting some disturbance while playing from wsl because of wsl audio bridge so for now i ran the program natively 
will fix this soon 


to play the song 

cd termix and
 ./target/x86_64-pc-windows-gnu/release/termix.exe "$(wslpath -w 'Songs/O Rangrez.mp3')"


## Author

Oxyraptor



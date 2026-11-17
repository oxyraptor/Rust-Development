# Termix

Termix is a Rust-based music player project under development.

## Recent Learning

The project now demonstrates Rust variable bindings:

- Variables are immutable by default.
- Use `mut` when a variable needs to be changed after it is created.
- Immutable and mutable bindings help make changes to program state explicit.

- Signed and unsigned variables 
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
## Current Features

- Music playback status output
- Album artwork support planned
- Playlist management planned

## Run

```bash
cargo run
```

## Development

Build the project with:

```bash
cargo build
```

## Author

Oxyraptor

mod sample {
    pub fn play_song() {
        println!("Playing Song");
    }
}

pub fn do_something() -> () {
    sample::play_song();
}
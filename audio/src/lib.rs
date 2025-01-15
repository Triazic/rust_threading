#[unsafe(no_mangle)]
pub fn audio_tick() -> () {
    println!("Audio tick 19 at {:?}", std::time::SystemTime::now());
}
use std::thread;
use std::time::Duration;

fn audio_tick() {
    loop {
        println!("Audio tick at {:?}", std::time::SystemTime::now());
        thread::sleep(Duration::from_millis(0)); // Simulate work
    }
}

fn input_tick() {
    loop {
        println!("Input tick at {:?}", std::time::SystemTime::now());
        thread::sleep(Duration::from_millis(0)); // Simulate work
    }
}

fn game_tick() {
    loop {
        println!("Game tick at {:?}", std::time::SystemTime::now());
        thread::sleep(Duration::from_millis(0)); // Simulate work
    }
}

fn render_tick() {
    loop {
        println!("Render tick at {:?}", std::time::SystemTime::now());
        thread::sleep(Duration::from_millis(0)); // Simulate work
    }
}

fn ai_tick() {
    loop {
        println!("AI tick at {:?}", std::time::SystemTime::now());
        thread::sleep(Duration::from_millis(0)); // Simulate work
    }
}

fn main() {
    let handles = vec![
        thread::spawn(audio_tick),
        thread::spawn(input_tick),
        thread::spawn(game_tick),
        thread::spawn(render_tick),
        thread::spawn(ai_tick),
    ];

    // Wait for all threads to finish (in this case, they never do)
    for handle in handles {
        let _ = handle.join(); // Join will block forever as threads loop infinitely
    }
}

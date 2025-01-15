#![allow(unused)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn audio_tick_with_reload() {
    loop {
      // spam load the .so over and over.
      // how to improve?
      let audio_lib_path = std::env::current_dir().unwrap().join("audio").join("target").join("debug").join("libaudio.so");
      assert!(audio_lib_path.exists());
      let audio_lib = unsafe { libloading::Library::new(audio_lib_path) }.unwrap();
      let audio_tick: libloading::Symbol<extern fn() -> ()> = unsafe { audio_lib.get(b"audio_tick") }.unwrap();
      audio_tick();
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

struct HotReloadState<'a> {
  audio_tick: Option<libloading::Symbol<'a, extern fn() -> ()>>,
}

fn main() {
    let hot_reload_state = HotReloadState {
        audio_tick: None,
    };

    let hot_reload_mutex = std::sync::Arc::new(std::sync::Mutex::new(hot_reload_state));

    let handles = vec![
        thread::spawn(audio_tick_with_reload),
        // thread::spawn(input_tick),
        // thread::spawn(game_tick),
        // thread::spawn(render_tick),
        // thread::spawn(ai_tick),
    ];

    // Wait for all threads to finish (in this case, they never do)
    for handle in handles {
        let _ = handle.join(); // Join will block forever as threads loop infinitely
    }
}

#![allow(unused)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn audio_tick_with_reload(mutex: Arc<Mutex<HotReloadState<'_>>>) {
    loop {
      let lock = mutex.lock().unwrap();
      let blah = &lock.audio_tick;
      blah.as_ref().unwrap()();
      // match blah {
      //   Some(audio_tick) => {
      //     audio_tick();
      //   }
      //   None => {
      //     println!("oops, no audio tick");
      //   }
      // }
      // spam load the .so over and over.
      // how to improve?
      let audio_lib_path = std::env::current_dir().unwrap().join("audio").join("target").join("debug").join("libaudio.so");
      assert!(audio_lib_path.exists());
      let audio_lib = unsafe { libloading::Library::new(audio_lib_path) }.unwrap();
      let audio_tick: libloading::Symbol<extern fn() -> ()> = unsafe { audio_lib.get(b"audio_tick") }.unwrap();
      audio_tick();
    }
}

fn input_tick(mutex: Arc<Mutex<HotReloadState<'_>>>) {
    loop {
        let lock = mutex.lock().unwrap();
        let blah = &lock.audio_tick;
        blah.as_ref().unwrap()();
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
    let hot_reload_mutex_audio_thread = hot_reload_mutex.clone();
    let hot_reload_mutex_input_thread = hot_reload_mutex.clone();

    let fns = vec![
      audio_tick_with_reload, 
      input_tick,
    ];

    let handles_2 = fns.into_iter().map(|f| {
      let mutex = hot_reload_mutex.clone();
      thread::spawn(move || {
        f(mutex);
      })
    });

    let handles = vec![
        thread::spawn(move || { audio_tick_with_reload(hot_reload_mutex_audio_thread); }),
        thread::spawn(move || { input_tick(hot_reload_mutex_input_thread); }),
        // thread::spawn(game_tick),
        // thread::spawn(render_tick),
        // thread::spawn(ai_tick),
    ];

    // Wait for all threads to finish (in this case, they never do)
    for handle in handles {
        let _ = handle.join(); // Join will block forever as threads loop infinitely
    }
}

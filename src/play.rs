use std::{
    sync::atomic::{AtomicBool, Ordering},
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
};

static DONE: AtomicBool = AtomicBool::new(false);

fn main() {
    let workers = (0..100u64).map(|mut i| std::thread::spawn(move|| loop {
        i = {
            let mut s = DefaultHasher::new();
            i.hash(&mut s);
            s.finish()
        };
        if i % 1000 == 0 && !DONE.compare_and_swap(false, true, Ordering::Relaxed) {
            return Some(i);
        }
        if DONE.load(Ordering::Relaxed) {
            return None;
        }
    })).collect::<Vec<_>>();
    
    let results = workers.into_iter().map(|it| it.join().unwrap())
        .collect::<Vec<_>>();
    
    let found_value = results.into_iter().find_map(|it| it).unwrap();
    println!("{}", found_value)
}
use std::mem::swap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // ANCHOR_END: Philosopher-eat
        println!("{} is trying to eat", &self.name);
        let left = self.left_fork.lock().unwrap();
        let right = self.right_fork.lock().unwrap();
        // ANCHOR: Philosopher-eat-end
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

pub(crate) fn run() {
    let size = PHILOSOPHERS.len();
    let (tx, rx) = mpsc::sync_channel(2 * size);

    // Create forks
    let forks: Vec<_> = (0..size).map(|_| Arc::from(Mutex::new(Fork))).collect();

    // Create philosophers
    let philosophers: Vec<_> = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let mut left_fork = forks[index % size].clone();
            let mut right_fork = forks[(index + 1) % size].clone();

            if index == forks.len() - 1 {
                swap(&mut left_fork, &mut right_fork);
            }

            let philosopher = Arc::new(Philosopher {
                name: name.to_string(),
                left_fork,
                right_fork,
                thoughts: tx.clone(),
            });

            philosopher
        })
        .collect();

    // Make them think and eat
    // Eat and think
    for philosopher in philosophers {
        for _ in 0..10 {
            let phi = philosopher.clone();
            thread::spawn(move || {
                for _ in 0..10 {
                    phi.eat();
                    phi.think()
                }
            });
        }
    }

    drop(tx);
    // Output their thoughts
    let mut total = 0;
    for rc in &rx {
        println!(":::: => {}", rc);
        total += 1;
    }
    println!("Total thoughts {total}");
}

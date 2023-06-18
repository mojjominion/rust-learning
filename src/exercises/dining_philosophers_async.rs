use std::mem::swap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time::sleep;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .await
            .unwrap();
    }

    async fn eat(&self) {
        // ANCHOR_END: Philosopher-eat
        println!("{} is trying to eat", &self.name);
        let left = self.left_fork.lock().await;
        let right = self.right_fork.lock().await;
        // ANCHOR: Philosopher-eat-end
        println!("{} is eating...", &self.name);
        sleep(Duration::from_millis(10)).await;
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn creat_philosophers(
    forks: &Vec<Arc<Mutex<Fork>>>,
) -> (Vec<Arc<Philosopher>>, mpsc::Receiver<String>) {
    let size = PHILOSOPHERS.len();
    let (tx, rx) = mpsc::channel(10);

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

    (philosophers, rx)
    // tx is dropped here, so we don't need to explicitly drop it later
}

#[tokio::main]
pub(crate) async fn run() {
    let size = PHILOSOPHERS.len();
    // Create forks
    let forks: Vec<_> = (0..size).map(|_| Arc::from(Mutex::new(Fork))).collect();

    // Create philosophers
    let (philosophers, mut rx) = creat_philosophers(&forks);

    // Make them think and eat
    // Eat and think
    for philosopher in philosophers {
        for _ in 0..10 {
            let phi = philosopher.clone();
            tokio::spawn(async move {
                for _ in 0..10 {
                    phi.eat().await;
                    phi.think().await;
                }
            });
        }
    }

    // Output their thoughts
    let mut total = 0;
    while let Some(rc) = rx.recv().await {
        println!(":::: => {}", rc);
        total += 1;
    }
    println!("Total thoughts {total}");
}

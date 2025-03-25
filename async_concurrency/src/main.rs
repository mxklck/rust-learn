use std::pin::Pin;
use std::pin::pin;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use trpl;
use trpl::Either;
use trpl::Either::Left;
use trpl::Either::Right;
use trpl::join_all;

async fn tasks() {
    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };
    trpl::join(fut1, fut2).await;
}

async fn message_passing() {
    // Within a given async block, the order in which await keywords
    // appear in the code is also the order in which they’re executed
    // when the program runs.
    let (tx, mut rx) = trpl::channel(); // we now have a mutable rx!
    let ty = tx.clone();

    // allocate on heap with pointers
    let initial_messages = pin!(create_messages(tx));
    let receiver = pin!(receive_messages(&mut rx));

    let secondary_messages = pin!(async move {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            ty.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
    let now = Instant::now();

    // output of our futures is the unit type () i.e. void
    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
        vec![initial_messages, secondary_messages, receiver];
    trpl::join_all(futures).await;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    // only takes 2 seconds (even though there are 8*0.5s statements) due to concurrency!
}

// takes ownership of tx (the sender) - could also re-write using async move {...}
// async blocks return a trait (a future) - this is slightly confusing...
async fn create_messages(tx: UnboundedSender<String>) {
    let vals = vec![
        String::from("Hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }
} // tx gets dropped here as we take ownership, hence receive message can end

async fn receive_messages(rx: &mut UnboundedReceiver<String>) {
    while let Some(value) = rx.recv().await {
        println!("received '{value}'");
    }
}

fn super_slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms)); // blocks current thread for ms milliseconds
    println!("'{name}' ran for '{ms}'ms");
}

async fn race() {
    let a = async {
        println!("'a' has started");
        super_slow("a", 30);
        trpl::yield_now().await;
        super_slow("a", 10);
        trpl::yield_now().await;
        super_slow("a", 20);
        trpl::yield_now().await;
        // trpl::sleep(Duration::from_millis(50)).await; // on hitting this await it moves onto b
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        super_slow("b", 75);
        trpl::yield_now().await;
        super_slow("b", 10);
        trpl::yield_now().await;
        super_slow("b", 15);
        trpl::yield_now().await;
        super_slow("b", 350);
        trpl::yield_now().await;
        // trpl::sleep(Duration::from_millis(50)).await; // on hitting this await it re-starts work in a
        println!("'b' finished."); // then finishes b
    };
    // comments are without any fancy interleaving
    trpl::join(a, b).await;

    let slow = async {
        trpl::sleep(Duration::from_secs(1)).await;
        "I finished!" // no semicolon here - it returns the string "I finished!"
    };

    match timeout(slow, Duration::from_secs(2)).await {
        Ok(message) => println!("Got message '{message}'"),
        Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
    };
}

async fn timeout<F: Future>(future: F, max_duration: Duration) -> Result<F::Output, Duration> {
    let timer = trpl::sleep(max_duration); // this is a future

    // opens the Either using a match and converts it to and Result
    let result = trpl::race(future, timer).await;
    match result {
        Left(left) => Ok(left),
        Right(_) => Err(max_duration), // ignore unit type () from timer finishing and instead return Err(max_duration)
    }
}

fn main() {
    trpl::run(race());
    // trpl::run(message_passing());
    // trpl::run(tasks());
}

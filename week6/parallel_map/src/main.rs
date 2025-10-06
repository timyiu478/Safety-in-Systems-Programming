use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);

    let (in_sender, in_receiver) = crossbeam_channel::unbounded();
    let (out_sender, out_receiver) = crossbeam_channel::unbounded();

    let mut threads = Vec::new();
    for _ in 0..num_threads {
       let receiver = in_receiver.clone();
       let sender = out_sender.clone();
       threads.push(thread::spawn(move || {
            while let Ok(msg) = receiver.recv() {
                let (i, next_num) = msg;
                sender.send((i, f(next_num))).unwrap();
            }
       }));
    }

    drop(out_sender);
    drop(in_receiver);

    for (i, input) in input_vec.into_iter().enumerate() {
        in_sender.send((i, input)).expect("Failed to send item to worker threads");
    }

    // break the loop in worker threads
    drop(in_sender);

    // the loop will end when all out senders are dropped and the channel is closed
    while let Ok(out_msg) = out_receiver.recv() {
        let (i, out_item) = out_msg;
        output_vec[i] = out_item;
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}

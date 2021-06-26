use std::{
    thread,
    time::{self, Duration},
};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Error build tokio-runtime.");
    let mut handles = Vec::new();
    let start = time::Instant::now();
    for i in 0..15 {
        let handle = rt.spawn(async move { my_async_function(i).await });
        handles.push(handle);
    }
    dbg!(&start.elapsed());
    for handle in handles {
        let double_i = rt.block_on(async { handle.await.unwrap() });
        dbg!(double_i);
    }
    dbg!(start.elapsed());
}

async fn my_async_function(a: u32) -> u32 {
    thread::sleep(Duration::from_secs(3));
    dbg!(a);
    a + a
}

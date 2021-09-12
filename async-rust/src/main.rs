use std::thread;
use std::time::Duration;
use std::time::Instant;
use async_std::task;

// fn upload_media(){
//     println!("Uploading media starts");
//     thread::sleep(Duration::from_millis(500));
//     println!("Uploading media ends");
// }

// fn related_post_details(){
//     println!("Posting other details starts");
//     thread::sleep(Duration::from_millis(50));
//     println!("Posting other details ends");
// }

// fn add_post(){
//     // let thread_1 = thread::spawn(|| {
        
//         upload_media();
        
//     // });

//     // let thread_2 = thread::spawn(|| {
        
//         related_post_details();
//     // });
//     // thread_1.join().expect("Thread one panicked")
// }

// fn main() {
//     let start = Instant::now();
//     add_post();
//     let elapsed = start.elapsed();
//     println!("Time elapsed {:?}", elapsed);
// }

use futures::executor::block_on;

// async fn hello_world(){
//     println!("test");
// }

// fn main(){
//     block_on(hello_world());
// }

fn clean_room(){
// takes 20 minutes
println!("Started cleaning room");
thread::sleep(Duration::from_millis(2000));
println!("Finished cleaning room");
}
//Async returns future

async fn make_tea(){
// takes 20 minutes
println!("Started making tea");
task::sleep(Duration::from_millis(1000)).await;
println!("Finished making tea");
}

async fn wash_dishes(){
// takes 20 minutes
println!("Started washing dishes");
task::sleep(Duration::from_millis(1000)).await;
println!("Finished washing dishes");
}

async fn do_work_efficiently(){
    task::spawn(make_tea()); //async
    
    clean_room();   // 2sec
    
    let result = wash_dishes().await;  //2 sec
    make_tea();
}

fn main() {
    let start = Instant::now();//00:02
    block_on(do_work_efficiently());


    let elapsed = start.elapsed(); // 00:56 
    println!("Time taken : {:?}", elapsed);
}
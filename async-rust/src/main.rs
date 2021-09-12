use std::thread;
use std::time::Duration;
use std::time::Instant;

fn upload_media(){
    println!("Uploading media starts");
    thread::sleep(Duration::from_millis(500));
    println!("Uploading media ends");
}

fn related_post_details(){
    println!("Posting other details starts");
    thread::sleep(Duration::from_millis(50));
    println!("Posting other details ends");
}

fn add_post(){
    // let thread_1 = thread::spawn(|| {
        
        upload_media();
        
    // });

    // let thread_2 = thread::spawn(|| {
        
        related_post_details();
    // });
    // thread_1.join().expect("Thread one panicked")
}

fn main() {
    let start = Instant::now();
    add_post();
    let elapsed = start.elapsed();
    println!("Time elapsed {:?}", elapsed);
}

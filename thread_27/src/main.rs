use std::thread;
use std::time::Duration;

fn main() {
    /*
        创建线程 std::thread::spawn();
    */

    // thread::spawn(||{
    //     for i in 1..10{
    //         println!("子线程{}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 1..5{
    //     println!("主线程{}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // thread::sleep()会让线程睡眠一段时间，某个线程睡眠的时候，有让出cpu,可以让不同的线程交替执行，要看操作系统如何调度线程。

    // Join方法

   let handler = thread::spawn(||{
        for i in 1..10{
            println!("子线程{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("主线程{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}

# Rust 스레드간 메모리 공유

스레드간 통신하는 방법에는 채널을 사용하는 방법 말고도 메모리 공유하는 방법이 존재한다

하지만 다수의 스레드가 동시에 하나의 메모리 구역에 접근한다면 race condition(경쟁 상태)이 일어나서 안정적인 결과를 보장하지 않는다

따라서 mutex라는 기법이 사용되는데 Rust에서 이를 지원한다
mutex는 간단하게 설명하면 한 스레드가 한 메모리 구역에 접근하고 있다면 다른 스레드는 그 메모리 구역에 접근 할 수 없다

```Rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let data = Mutex::new(10);
    
    let handle = thread::spawn(move || {
        let mut num = data.lock().unwrap();
    	num += 1;
    });
    
    handle.join();
}
```

mutex를 위와 같이 사용할 수 있다
다만 문제점이 존재하는데 Mutex객체의 소유권을 오직 하나의 스레드가 가져간다는 것이다
따라서 여러 스레드에서 소유권을 가져가게 하기위해 참조-세기 방식을 사용하는 Rc객체를 생각할 수 있다

하지만 Rc객체에 문제점이 있다면 멀티 스레드 환경에서 안정적이지 않다
(Rc객체 내부에서 race condition 등의 문제가 발생하기 때문이다)

따라서 멀티 스레드에 안전한 참조-세기 방식이 필요하는데 이에대해 Rust에서 제공하는 것이 Arc<T>객체이다

이를 이용해 프로그래밍을 한다면 아래와 같이 할 수 있다

```Rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {    
    let data = Arc::new(Mutex::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *(data.lock().unwrap()));

}
```



그리고 Mutex가 보편적인 rust의 객체와는 다르게 혹은 RcCell 처럼 내부 값에 대해 가변성을 가진다

Mutex를 활용하면 위에서 처럼 메모리를 공유할 수 있게 되지만 서로 스레드에 lock을 걸어서 서로를 무한정 기다리는 데드락이라는 문제가 발생할 수 있으므로 유의해야 한다
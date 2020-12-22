use std::sync::Mutex;


fn main() {
    println!("Hello, world!");
    let  m = Mutex::new(5);
    {
        let mut mum = m.lock().unwrap();
        *mum = 6;
    }
    println!("m = {:?}",m)
}

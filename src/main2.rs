// use std::sync::{Mutex,Arc};
// use std::{thread};
//
// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//     // println!('handles {:?}',handles);
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//     println!("Result: {}", *counter.lock().unwrap());
// }

use std::option::Option::Some;

// fn main(){
//     let mut stack = Vec::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     stack.push(4);
//     while let Some(top) =stack.pop(){
//         println!("{}",top);
//     }
//     let v  = vec!['a','c','b'];
//     for (index,value) in v.iter().enumerate() {
//         println!("{} is at index {}",value,index)
//     }
// }
// fn main(){
//     let x = 'c';
//     match x {
//         'c'..='j'=> println!("early ASCII letter"),
//         'c'..='z'=> println!("late ASCII letter"),
//         _=> println!("something else"),
//     }
// }
// struct Pont{
// x:i32,
//     y:i32
// }
// fn main (){
//     let p = Pont{
//         x:0,
//         y:7
//     };
//     let Pont {x:a,y:b} = p;
//     println!("{}",a);
//     println!("{}",b);
//     let numbers = (2,4,6,8,32);
//     match numbers {
//         (first,_,third,_,fifth)=>{
//             println!("Some numbers :{}-{}-{}",first,third,fifth);
//         }
//     }
// }

fn main(){
    let  x = 4;
    let y = false;
    match x{
        4|5|6 if y => println!("yes"),
        _=> println!("no"),
    }
}
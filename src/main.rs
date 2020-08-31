use arraydeque::ArrayDeque;
use arraydeque::Wrapping;
use std::convert::TryFrom;

fn print_array_deque<A: arraydeque::Array, B: arraydeque::behavior::Behavior>(
    q: &arraydeque::ArrayDeque<A, B>,
) where
    <A as arraydeque::Array>::Item: std::fmt::Display,
{
    println!("ArrayDeque: capacity: {}, len: {}", q.capacity(), q.len());
    print!("  [");
    let lasti = q.len() - 1;
    for (i, element) in q.iter().enumerate() {
        if i == lasti {
            println!("{}]", element)
        } else {
            print!("{}, ", element)
        }
    }
}

/// Encode data using consistent overhead byte stuffing (COBS)
///
/// Assumes everything in the buffer is unencoded data, and message is < 254 bytes long
///
fn cobs_encode(q: &mut arraydeque::ArrayDeque<[u8; 64], Wrapping>) {
    q.push_front(0u8);
    q.push_back(0u8);
    let mut i_last_zero: usize = 0;
    let qlen = q.len();
    for i in 1..qlen {
        if let Some(&0) = q.get(i) {
            match q.get_mut(i_last_zero) {
                Some(last_zero_el) => match u8::try_from(i - i_last_zero) {
                    Ok(last_zero_el_next) => {
                        *last_zero_el = last_zero_el_next;
                        i_last_zero = i;
                    }
                    Err(why) => panic!("{:?}", why),
                },
                None => panic!("i_last_zero wasn't accessible!!!"),
            }
        }
    }
}

fn cobs_decode(q: &mut arraydeque::ArrayDeque<[u8; 64], Wrapping>) {
    let mut i_zero: usize = 0;
    let i_last = q.len() - 1;
    while i_zero < i_last {
        match q.get_mut(i_zero) {
            Some(i_zero_val) => {
                i_zero += usize::from(*i_zero_val);
                *i_zero_val = 0u8;
            }
            None => panic!("i_zero wasn't accessible!!!"),
        }
    }
    q.pop_front();
}

fn main() {
    const CAPACITY: usize = 64;
    let mut q: ArrayDeque<[u8; CAPACITY], Wrapping> = ArrayDeque::new();
    q.push_back(0x22);
    q.push_back(0x4);
    println!("Capacity: {}", q.capacity());
    println!("len: {}", q.len());
    println!("is_empty: {}", q.is_empty());
    println!("is_full: {}", q.is_full());
    println!("contains 0: {}", q.contains(&0));
    println!("contains 0x22: {}", q.contains(&0x22));
    if let Some(val) = q.front() {
        println!("front: {}", val);
    }
    if let Some(val) = q.back() {
        println!("back: {}", val);
    }
    match q.get(0) {
        Some(val) => println!("Element 0: {}", val),
        None => println!("No element 0"),
    }
    match q.get(4) {
        Some(val) => println!("Element 4: {}", val),
        None => println!("No element 4"),
    }
    print_array_deque(&q);

    q.push_back(0x0);
    q.push_back(0x22);
    q.push_back(0x3);
    q.push_back(0x0);
    q.push_back(0x9);
    q.push_back(0xA);
    print_array_deque(&q);
    cobs_encode(&mut q);
    print_array_deque(&q);
    cobs_decode(&mut q);
    print_array_deque(&q);
}

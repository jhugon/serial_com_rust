use arraydeque::ArrayDeque;
use arraydeque::Wrapping;
use serial_com_rust::*;

fn main() -> SerialComResult<()> {
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
    let orig_q_1 = q.clone();
    print_array_deque(&q);
    cobs_encode(&mut q)?;
    print_array_deque(&q);
    cobs_decode(&mut q)?;
    q.pop_back();
    print_array_deque(&q);
    println!("q1 equal: {}", q == orig_q_1);

    q.clear();
    rand_array_deque(&mut q, 32);
    let orig_q_2 = q.clone();
    print_array_deque(&q);
    cobs_encode(&mut q)?;
    print_array_deque(&q);
    cobs_decode(&mut q)?;
    q.pop_back();
    print_array_deque(&q);
    println!("q2 equal: {}", q == orig_q_2);

    Ok(())
}

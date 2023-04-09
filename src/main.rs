use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let orders: Vec<Vec<u16>> = read_orders();
    let mut coffe_act = 0;
    let mut threads = vec![];
    for order in orders {
        let order_act: Vec<u16> = order.clone();
        coffe_act = coffe_act + 1;
        let handle = thread::spawn(move || {
            println!("Preparing coffee {}",coffe_act);
            handle_order(order_act);
            println!("Coffee {} has been done",coffe_act);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Error joining a thread");
    }
    println!("All coffees have been prepared");
}

fn read_orders() -> Vec<Vec<u16>> {
    let file = fs::read_to_string(std::path::PathBuf::from("src/orders.txt"))
        .expect("Failed to read the file");
    let lines: Vec<&str> = file.split('\n').collect();
    let mut orders:Vec<Vec<u16>> = Vec::new(); 
    for line in lines {
        let line_orders: Vec<u16> = line
            .split(',')
            .map(|num| num.parse().expect("Ingredient must be integer"))
            .collect();
        orders.push(line_orders);  
    }
    orders  
}

fn handle_order(order: Vec<u16>) {
    for amount in order{
        thread::sleep(Duration::from_millis(amount.into()))
    }
    println!("Cafe listo")
}
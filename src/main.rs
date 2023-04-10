use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tp1::containers::containers::Container;
use tp1::containers::enum_containers::ContainerTypes;

fn main() {
    let orders: Vec<Vec<u64>> = read_orders();
    let mut coffe_act:u64 = 0;
    let mut threads = vec![];
    let mut containers_vec = vec![];
    for container_type in ContainerTypes::iter() {
        let container = Arc::new(Mutex::new(Container::new(15, container_type)));
        containers_vec.push(container);
    }
    for order in orders {
        let order_act: Vec<u64> = order.clone();
        let containers_ref = Arc::clone(&Arc::new(containers_vec.clone()));
        coffe_act = coffe_act + 1;
        let handle = thread::spawn(move || {
            println!("Preparing coffee {}",coffe_act);
            handle_order(order_act, containers_ref);
            println!("Coffee {} has been done",coffe_act);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Error joining a thread");
    }

    println!("All coffees have been prepared");
}

fn read_orders() -> Vec<Vec<u64>> {
    let file = fs::read_to_string(std::path::PathBuf::from("src/orders.txt"))
        .expect("Failed to read the file");
    let lines: Vec<&str> = file.split('\n').collect();
    let mut orders:Vec<Vec<u64>> = Vec::new(); 
    for line in lines {
        let line_orders: Vec<u64> = line
            .split(',')
            .map(|num| num.parse().expect("Ingredient must be integer"))
            .collect();
        orders.push(line_orders);  
    }
    orders  
}

fn handle_order(order: Vec<u64>, containers: Arc<Vec<Arc<Mutex<Container>>>>) {
    for (index,amount) in order.iter().enumerate(){
        let cont_act = containers[index].lock().expect("Error trying to use container");
        cont_act.serve(*amount);
        thread::sleep(Duration::from_millis(*amount))
    }
    println!("Cafe listo")
}
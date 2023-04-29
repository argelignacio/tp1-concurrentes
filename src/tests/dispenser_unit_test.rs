 // Test unitarios
#[cfg(test)] pub mod unit_tests_dispenser{
   use std::sync::{Mutex, Arc};
   use crate::dispensers::dispensers_class::Dispensers;
   use crate::mocks::contenedor_mock::Container;
   #[test]
   fn test_prepare_coffee_calls_well(){
      let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![Arc::new(Mutex::new(Container::new()))];
      let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> = Arc::clone(&Arc::new(containers_vec.clone()));
      let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
      dispenser.lock().unwrap().prepare(vec![1], 1, false);
      assert_eq!(containers_vec[0].lock().unwrap().amount, 1);
      assert_eq!(containers_vec[0].lock().unwrap().report, false);
      dispenser.lock().unwrap().prepare(vec![20], 1, true);
      assert_eq!(containers_vec[0].lock().unwrap().amount, 20);
      assert_eq!(containers_vec[0].lock().unwrap().report, true);
   }
} 




 
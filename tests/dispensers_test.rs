#[cfg (test)]

mod tests {
    use std::{sync::{Mutex, Arc}, vec};

    use mockall::mock;
    use tp1::{dispensers::dispensers_class::Dispensers, containers::containers_class::Container};


    mock! {
        pub Container {
            fn serve(&self, arg1: u64, arg2: bool) -> Result<(), String>;
        }
    }

    #[test] 
    fn test_prepare_coffe() {
        let mut mock_container: MockContainer = MockContainer::new();    
        
        let vector = vec![Arc::new(Mutex::new(mock_container))];
        let arc_vec = Arc::new(vector);
        mock_container.expect_serve()
            .withf(|arg1: u64, arg2: bool| arg1 == 1 && arg2 == true)
            .times(1)
            .returning(|_, _| Ok(()));

        let calculator = Dispensers::new(arc_vec,3);
        
    }
}
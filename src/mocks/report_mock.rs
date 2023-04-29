#[cfg(test)]
pub mod ReportMock {
    use std::{
        sync::{Arc, Mutex, RwLock},
        thread,
        time::Duration,
    };
    
    use crate::containers::containers_class::Container;
    
    const PERIOD_TO_REPORT: i32 = 1000;
    pub struct ReportMaker {
        report: bool,
    }
    
    impl ReportMaker {
        pub fn new() -> Self {
            Self { report: True }
        }
        /// Esta función inicia el proceso de preparación de las ordenes y manejar los threads resultantes de las mismas.
        pub fn start_reports(
            &self,
            containers: Arc<Mutex<Vec<Arc<Mutex<Container>>>>>,
        ) -> thread::JoinHandle<()> {

            thread::spawn(move ||  {
                
            })
        }
        /// Esta función se encarga de detener los reportes de los contenedores.
        pub fn stop_reports(&mut self) {
           self.report = false
        }
    }
    
}

#[cfg(test)]
pub mod ReportMock {
    use std::{
        sync::{Arc, Mutex, RwLock},
        thread,
        time::Duration,
    };

    use crate::mocks::contenedor_mock::Container;
    #[derive(Clone)]
    pub struct ReportMaker {
        pub report: bool,
        pub call_start_report: bool,
    }

    impl ReportMaker {
        pub fn new() -> Self {
            Self {
                call_start_report: false,
                report: true,
            }
        }
        /// Esta función inicia el proceso de preparación de las ordenes y manejar los threads resultantes de las mismas.
        pub fn start_reports(
            &mut self,
            containers1: Arc<Mutex<Vec<Arc<Mutex<Container>>>>>,
        ) -> thread::JoinHandle<()> {
            self.call_start_report = true;
            thread::spawn(move || {})
        }
        /// Esta función se encarga de detener los reportes de los contenedores.
        pub fn stop_reports(&mut self) {
            self.report = false
        }
    }
}

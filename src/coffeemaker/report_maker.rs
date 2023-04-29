use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use crate::containers::containers_class::Container;

const PERIOD_TO_REPORT: i32 = 1000;
pub struct ReportMaker {
    report: Arc<RwLock<bool>>,
}

impl ReportMaker {
    pub fn new() -> Self {
        Self { report: Arc::new(RwLock::new(true)) }
    }
    pub fn start_reports(
        &self,
        containers: Arc<Mutex<Vec<Arc<Mutex<Container>>>>>,
    ) -> thread::JoinHandle<()> {
        let report_flag = Arc::clone(&self.report);
        let report: thread::JoinHandle<()> = thread::spawn(move || loop {
            let mut val_act = false;
            if let Ok(flag) = report_flag.read() {
                val_act = *flag;
            }
            if !val_act {
                break;
            } else {
                thread::sleep(Duration::from_millis(PERIOD_TO_REPORT as u64));
                if let Ok(containers) = containers.lock() {
                    for container in containers.iter() {
                        if let Ok(container) = container.lock() {
                            container.periodic_reports();
                        } else {
                            println!("Error al acceder al contenedor");
                        }
                    }
                } else {
                    println!("Error al acceder a los contenedores");
                }
            }
        });
        report
    }
    /// Esta función se encarga de detener los reportes de los contenedores.
    pub fn stop_reports(&mut self) {
        if let Ok(mut report) = self.report.write() {
            *report = false
        }
    }
}

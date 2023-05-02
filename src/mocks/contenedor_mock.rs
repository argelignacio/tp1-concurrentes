#[cfg(test)]
/// Este struct representa un contenedor de un ingrediente, el cual tiene una capacidad máxima y un tipo de ingrediente.
pub struct Container {
    pub amount: u64,
    pub report: bool,
}
#[cfg(test)]
/// El contenedor necesita conocer su capacidad maxima, el tipo de ingrediente que contiene y la
/// capacidad porcentual a la que debe imprimir alertas.
impl Container {
    pub fn new() -> Self {
        Self {
            amount: 0,
            report: true,
        }
    }

    /// Esta función consume el ingrediente y en caso de no tener suficiente, maneja si recarga o lanza error por insuficiencia.
    /// En caso de que el amount sea mayor a la capacidad del contenedor, se lanza un error.
    /// En caso de que el reporte sea true, se imprime el reporte de consumos.
    pub fn serve(&mut self, amount: u64, report: bool) -> Result<(), String> {
        self.amount = amount;
        self.report = report;

        Ok(())
    }
    /// Esta función imprime el reporte de consumos del contendedor.
    pub fn periodic_reports(&self) {}
}

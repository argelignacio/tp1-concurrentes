#[cfg (test)]

mod tests {
    use tp1::containers::containers_class::Container;
    use tp1::containers::enum_containers::ContainerTypes;

    #[test] 
    fn test_serve_cafe() {
        let mut container = Container::new(100, ContainerTypes::Cafe, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(90, false), Err(String::from("Nos quedamos sin Cafe.")));
        assert_eq!(container.serve(3000, false), Err(String::from("La cantidad de Cafe es mayor a la capacidad del contenedor.")));
    }

    #[test] 
    fn test_serve_agua() {
        let mut container = Container::new(100, ContainerTypes::Agua, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(3000, false), Err(String::from("La cantidad de Agua es mayor a la capacidad del contenedor.")));
    }

    #[test] 
    fn test_serve_cacao() {
        let mut container = Container::new(100, ContainerTypes::Cacao, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Err(String::from("Nos quedamos sin Cacao.")));
        assert_eq!(container.serve(3000, false), Err(String::from("La cantidad de Cacao es mayor a la capacidad del contenedor.")));
    }

    #[test]
    fn test_serve_espuma() {
        let mut container = Container::new(100, ContainerTypes::Espuma, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(90, false), Err(String::from("Nos quedamos sin Espuma.")));
        assert_eq!(container.serve(3000, false), Err(String::from("La cantidad de Espuma es mayor a la capacidad del contenedor.")));
    }


}
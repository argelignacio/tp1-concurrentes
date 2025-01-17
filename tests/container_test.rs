#[cfg(test)]

mod tests {
    use tp1::containers::containers_class::Container;
    use tp1::containers::enum_containers::ContainerTypes;

    #[test]
    fn test_serve_cafe() {
        let mut container = Container::new(100, ContainerTypes::Cafe, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
    }
    #[test]
    fn test_serve_cafe_insuficiente() {
        let mut container = Container::new(100, ContainerTypes::Cafe, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(
            container.serve(90, false),
            Err(String::from("ERR: Nos quedamos sin Cafe."))
        );
    }
    #[test]
    fn test_serve_cafe_overflow() {
        let mut container = Container::new(100, ContainerTypes::Cafe, 10.0);
        assert_eq!(
            container.serve(3000, false),
            Err(String::from(
                "ERR: La cantidad de Cafe es mayor a la capacidad del contenedor."
            ))
        );
    }

    #[test]
    fn test_serve_agua_overflow() {
        let mut container = Container::new(100, ContainerTypes::Agua, 10.0);
        assert_eq!(
            container.serve(3000, false),
            Err(String::from(
                "ERR: La cantidad de Agua es mayor a la capacidad del contenedor."
            ))
        );
    }

    #[test]
    fn test_serve_agua_infinita() {
        let mut container = Container::new(100, ContainerTypes::Agua, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
        assert_eq!(container.serve(99, false), Ok(()));
    }

    #[test]
    fn test_serve_cacao() {
        let mut container = Container::new(100, ContainerTypes::Cacao, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
    }
    #[test]
    fn test_serve_cacao_insuficiente() {
        let mut container = Container::new(100, ContainerTypes::Cacao, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(
            container.serve(90, false),
            Err(String::from("ERR: Nos quedamos sin Cacao."))
        );
    }
    #[test]
    fn test_serve_cacao_overflow() {
        let mut container = Container::new(100, ContainerTypes::Cacao, 10.0);
        assert_eq!(
            container.serve(3000, false),
            Err(String::from(
                "ERR: La cantidad de Cacao es mayor a la capacidad del contenedor."
            ))
        );
    }

    #[test]
    fn test_serve_espuma() {
        let mut container = Container::new(100, ContainerTypes::Espuma, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(
            container.serve(3000, false),
            Err(String::from(
                "ERR: La cantidad de Espuma es mayor a la capacidad del contenedor."
            ))
        );
    }
    #[test]
    fn test_serve_espuma_insuficiente() {
        let mut container = Container::new(100, ContainerTypes::Espuma, 10.0);
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(container.serve(50, false), Ok(()));
        assert_eq!(
            container.serve(90, false),
            Err(String::from("ERR: Nos quedamos sin Espuma."))
        );
    }
    #[test]
    fn test_serve_espuma_overflow() {
        let mut container = Container::new(100, ContainerTypes::Espuma, 10.0);
        assert_eq!(
            container.serve(3000, false),
            Err(String::from(
                "ERR: La cantidad de Espuma es mayor a la capacidad del contenedor."
            ))
        );
    }
}

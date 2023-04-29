#[cfg(test)]

mod tests {
    use tp1::refillers::enum_refillers::RefillerTypes;
    use tp1::refillers::refiller_class::Refiller;

    #[test]
    fn test_recharge() {
        let mut refiller = Refiller::new(100, RefillerTypes::Granos, 15.0);
        assert_eq!(refiller.recharge(50), 50);
        assert_eq!(refiller.recharge(50), 50);
        assert_eq!(refiller.recharge(50), 0);
    }
}

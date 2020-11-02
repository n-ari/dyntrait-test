#[cfg(test)]
mod tests {
    use base::*;

    #[test]
    fn it_can_parse() {
        let object = Object::new(3);
        let effect: Box<dyn Effect> = serde_json::from_str("{\"type\":\"ForAllEffect\"}").unwrap();
        effect.appliable(&object);
    }

    #[test]
    #[should_panic]
    fn it_cannot_parse() {
        let object = Object::new(3);
        let effect: Box<dyn Effect> =
            serde_json::from_str("{\"type\":\"SpecificEffect\",\"id\":3}").unwrap();
        effect.appliable(&object);
    }
}

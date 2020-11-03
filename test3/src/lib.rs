#[cfg(test)]
mod tests {
    use base::{Effect, ForAllEffect, Object};
    use dyntrait_de_wrapper::define_de_dyntrait_json;
    use ext::SpecificEffect;

    #[test]
    fn it_can_parse() {
        define_de_dyntrait_json!(de, Effect, [ForAllEffect]);
        let object = Object::new(3);
        let effect: Box<dyn Effect> = de("{\"type\":\"ForAllEffect\"}").unwrap();
        effect.appliable(&object);
    }

    #[test]
    fn it_also_can_parse() {
        define_de_dyntrait_json!(de, Effect, [ForAllEffect, SpecificEffect]);
        let object = Object::new(3);
        let effect: Box<dyn Effect> = de("{\"type\":\"SpecificEffect\",\"id\":3}").unwrap();
        effect.appliable(&object);
    }

    #[test]
    #[should_panic]
    fn it_cannot_parse() {
        define_de_dyntrait_json!(de, Effect, [ForAllEffect]);
        let object = Object::new(3);
        let effect: Box<dyn Effect> = de("{\"type\":\"SpecificEffect\",\"id\":3}").unwrap();
        effect.appliable(&object);
    }
}

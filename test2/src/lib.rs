use base::{Effect, ForAllEffect, Object};
use ext::SpecificEffect;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_parse() {
        let object = Object::new(3);
        let effect: Box<dyn Effect> = serde_json::from_str("{\"type\":\"ForAllEffect\"}").unwrap();
        effect.appliable(&object);
    }

    #[test]
    fn it_also_can_parse() {
        // なぜかこれがないと panic する
        let _: Box<dyn Effect> = Box::new(SpecificEffect::new(0));
        let object = Object::new(3);
        let effect: Box<dyn Effect> =
            serde_json::from_str("{\"type\":\"SpecificEffect\",\"id\":3}").unwrap();
        effect.appliable(&object);
    }
}

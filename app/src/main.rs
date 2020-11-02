use base::{Effect, ForAllEffect, Object};
use ext::SpecificEffect;

fn main() {
    let object = Object::new(3);

    // normal box
    let effect = Box::new(ForAllEffect::new());
    let json = serde_json::to_string(&effect).unwrap();
    println!("{} / {}", effect.appliable(&object), json);
    
    let effect = Box::new(SpecificEffect::new(2));
    let json = serde_json::to_string(&effect).unwrap();
    println!("{} / {}", effect.appliable(&object), json);
    
    let effect = Box::new(SpecificEffect::new(3));
    let json = serde_json::to_string(&effect).unwrap();
    println!("{} / {}", effect.appliable(&object), json);

    // dyn trait box
    let effect: Box<dyn Effect> = Box::new(ForAllEffect::new());
    let json = serde_json::to_string(&effect).unwrap();
    let parsed: Box<dyn Effect> = serde_json::from_str(&json).unwrap();
    println!("{} / {}", effect.appliable(&object), json);
    assert_eq!(effect.appliable(&object), parsed.appliable(&object));

    let effect: Box<dyn Effect> = Box::new(SpecificEffect::new(2));
    let json = serde_json::to_string(&effect).unwrap();
    let parsed: Box<dyn Effect> = serde_json::from_str(&json).unwrap();
    println!("{} / {}", effect.appliable(&object), json);
    assert_eq!(effect.appliable(&object), parsed.appliable(&object));

    let effect: Box<dyn Effect> = Box::new(SpecificEffect::new(3));
    let json = serde_json::to_string(&effect).unwrap();
    let parsed: Box<dyn Effect> = serde_json::from_str(&json).unwrap();
    println!("{} / {}", effect.appliable(&object), json);
    assert_eq!(effect.appliable(&object), parsed.appliable(&object));
}

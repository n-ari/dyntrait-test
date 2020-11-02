use base::{Effect, ForAllEffect, Object};
use ext::SpecificEffect;

fn main() {
    let object = Object::new(3);
    // normal box
    let effect = Box::new(ForAllEffect::new());
    println!(
        "{} / {}",
        effect.appliable(&object),
        serde_json::to_string(&effect).unwrap()
    );
    let effect = Box::new(SpecificEffect::new(2));
    println!(
        "{} / {}",
        effect.appliable(&object),
        serde_json::to_string(&effect).unwrap()
    );
    let effect = Box::new(SpecificEffect::new(3));
    println!(
        "{} / {}",
        effect.appliable(&object),
        serde_json::to_string(&effect).unwrap()
    );
    // dyn trait box
    let effect: Box<dyn Effect> = Box::new(ForAllEffect::new());
    println!(
        "{} / {}",
        effect.appliable(&object),
        serde_json::to_string(&effect).unwrap()
    );
    let effect: Box<dyn Effect> = Box::new(SpecificEffect::new(2));
    println!(
        "{} / {}",
        effect.appliable(&object),
        serde_json::to_string(&effect).unwrap()
    );
    let effect: Box<dyn Effect> = Box::new(SpecificEffect::new(3));
    println!(
        "{} / {}",
        effect.appliable(&object),
        serde_json::to_string(&effect).unwrap()
    );
}

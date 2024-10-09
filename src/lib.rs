use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Context = RefCell<HashMap<String, Box<dyn Any>>>;
type Ingredient = ();

trait Expression {
    fn interpret(&self, context: &Context);
}

struct MakeSandwich {
    ingredients: Vec<String>,
}

impl MakeSandwich {
    fn new() -> Self {
        Self {
            ingredients: Vec::new(),
        }
    }
}

struct Sandwich {
    ingredients: Vec<Ingredient>,
}

impl Sandwich {
    fn new() -> Self {
        Self {
            ingredients: Vec::new(),
        }
    }
}

impl Expression for MakeSandwich {
    fn interpret(&self, context: &Context) {
        let mut sandwich = Sandwich::new();
        for ing_str in &self.ingredients {
            let ingredient = context.borrow_mut().remove(ing_str)
                .expect("Can't make a sandwich without {ing_str}!");
            let ingredient = ingredient.downcast_ref::<Ingredient>()
                .expect("{ing_str} is not an Ingredient!");
            sandwich.ingredients.push(*ingredient);
        }
        context.borrow_mut().insert("sandwich".to_string(), Box::new(sandwich));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_sandwich_to_context() {
        // given
        let ingredients = vec!("cheese".to_string());
        let context: Context = RefCell::new(HashMap::new());
        context.borrow_mut().insert("cheese".to_string(), Box::new(()));
        let mut make_sandwich = MakeSandwich::new();
        make_sandwich.ingredients = ingredients;

        // when
        make_sandwich.interpret(&context);
        let sandwich = context.borrow_mut().remove("sandwich").unwrap();
        let sandwich = sandwich.downcast_ref::<Sandwich>().unwrap();
    }
}

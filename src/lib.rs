use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Context = RefCell<HashMap<String, Box<dyn Any>>>;
type Ingredient = ();

// interface
trait Expression {
    fn interpret(&self, context: &Context);
}

// POD
struct Sandwich {
    ingredients: Vec<Ingredient>,
}

impl Sandwich {
    fn new() -> Self {
        Self {
            ingredients: Vec::new(),
        }
    }

    fn eat(self) {
        println!("*sandwich eating noises*");
    }
}

// implementations of Expression
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

impl Expression for MakeSandwich {
    fn interpret(&self, context: &Context) {
        add_sandwich_to_context(context, Some(&self.ingredients));
        /*
        let mut sandwich = Sandwich::new();
        for ing_str in &self.ingredients {
            let ingredient = context.borrow_mut().remove(ing_str)
                .expect("Can't make a sandwich without {ing_str}!");
            let ingredient = ingredient.downcast_ref::<Ingredient>()
                .expect("{ing_str} is not an Ingredient!");
            sandwich.ingredients.push(*ingredient);
        }
        context.borrow_mut().insert("sandwich".to_string(), Box::new(sandwich));
        */
    }
}

fn add_sandwich_to_context(context: &Context, ingredients: Option<&Vec<String>>) {
        let mut sandwich = Sandwich::new();
        if let Some(ingredients) = ingredients {
            for ing_str in ingredients {
                let ingredient = context.borrow_mut().remove(ing_str)
                    .expect("Can't make a sandwich without {ing_str}!");
                let ingredient = ingredient.downcast_ref::<Ingredient>()
                    .expect("{ing_str} is not an Ingredient!");
                sandwich.ingredients.push(*ingredient);
            }
        }
        context.borrow_mut().insert("sandwich".to_string(), Box::new(sandwich));
}

struct EatSandwich {
}

impl EatSandwich {
    fn new() -> Self {
        Self {
        }
    }
}

impl Expression for EatSandwich {
    fn interpret(&self, context: &Context) {
        let sandwich = remove_sandwich_from_context(context);
        sandwich.expect("You expect to have a sandwich").eat();
    }
}

fn remove_sandwich_from_context(context: &Context) -> Option<Box<Sandwich>> {
    let mut sandwich = context.borrow_mut().remove("sandwich")?;
    sandwich.downcast::<Sandwich>().ok()
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

        // then
        let sandwich = context.borrow_mut().remove("sandwich").unwrap();
        let sandwich = sandwich.downcast_ref::<Sandwich>().unwrap();
    }

    #[test]
    fn should_eat_sandwich() {
        // given
        let context: Context = RefCell::new(HashMap::new());
        add_sandwich_to_context(&context, None);
        let mut eat_sandwich = EatSandwich::new();
        assert!(context.borrow().get("sandwich").is_some());

        // when
        eat_sandwich.interpret(&context);

        // then
        assert!(context.borrow().get("sandwich").is_none());
    }
}

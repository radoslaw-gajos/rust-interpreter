use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;

type Context = RefCell<HashMap<String, Box<dyn Any>>>;
type Ingredient = ();
type ExpResult = Result<(), Box<dyn Any>>;

// interface
trait Expression {
    fn interpret(&self, context: &Context) -> ExpResult;
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
    fn interpret(&self, context: &Context) -> ExpResult {
        add_sandwich_to_context(context, Some(&self.ingredients))
    }
}

fn add_sandwich_to_context(context: &Context, ingredients: Option<&Vec<String>>) 
    -> ExpResult {
        let mut sandwich = Sandwich::new();
        if let Some(ingredients) = ingredients {
            for ing_str in ingredients {
                let ingredient = context.borrow_mut().remove(ing_str);
                if let None = ingredient {
                    return Err(Box::new("Ingredient {ing_str} not found"));
                }
                let ingredient = ingredient.unwrap();
                let ingredient = ingredient.downcast_ref::<()>();
                let ingredient = ingredient.unwrap();
                
                sandwich.ingredients.push(*ingredient);
            }
        }
        context.borrow_mut().insert("sandwich".to_string(), Box::new(sandwich));
        Ok(())
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
    fn interpret(&self, context: &Context) -> ExpResult {
        let sandwich = remove_sandwich_from_context(context);
        if sandwich.is_none() {
            let err: ExpResult = Err(Box::new("Sandwich not found"));
            return err;
        }
        sandwich.expect("You expect to have a sandwich").eat();
        Ok(())
    }
}

fn remove_sandwich_from_context(context: &Context) -> Option<Box<Sandwich>> {
    let sandwich = context.borrow_mut().remove("sandwich");
    if sandwich.is_none() {
        return None;
    }
    sandwich.unwrap().downcast::<Sandwich>().ok()
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
        let result = make_sandwich.interpret(&context);

        // then
        let sandwich = context.borrow_mut().remove("sandwich").unwrap();
        sandwich.downcast_ref::<Sandwich>().unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn should_eat_sandwich() {
        // given
        let context: Context = RefCell::new(HashMap::new());
        add_sandwich_to_context(&context, None).unwrap();
        let eat_sandwich = EatSandwich::new();
        assert!(context.borrow().get("sandwich").is_some());

        // when
        let result = eat_sandwich.interpret(&context);

        // then
        assert!(result.is_ok());
        assert!(context.borrow().get("sandwich").is_none());
    }
}

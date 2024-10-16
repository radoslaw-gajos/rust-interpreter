use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;

pub type Context = RefCell<HashMap<String, Box<dyn Any>>>;
type Ingredient = ();
type ExpResult = Result<(), Box<dyn Any>>;

// interface
pub trait Expression {
    fn interpret(&self, context: &Context) -> ExpResult;
}

// POD
pub struct Sandwich {
    pub ingredients: Vec<Ingredient>,
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
pub struct And {
    expressions: Vec<Box<dyn Expression>>,
}

impl And {
    pub fn new(expressions: Vec<Box<dyn Expression>>) -> Self {
        Self {
            expressions,
        }
    }
}

impl Expression for And {
    fn interpret(&self, context: &Context) -> ExpResult {
        for exp in &self.expressions {
            let result = exp.interpret(context);
            if result.is_err() {
                return Err(Box::new("Expression failed"));
            }
        }
        Ok(())
    }
}

pub struct Or {
    expressions: Vec<Box<dyn Expression>>,
}

impl Or {
    pub fn new(expressions: Vec<Box<dyn Expression>>) -> Self {
        Self {
            expressions,
        }
    }
}

impl Expression for Or {
    fn interpret(&self, context: &Context) -> ExpResult {
        for exp in &self.expressions {
            let result = exp.interpret(context);
            if result.is_ok() {
                return Ok(())
            }
        }
        Err(Box::new("Expression failed"))
    }
}

pub struct Cry {
}

impl Cry {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Expression for Cry {
    fn interpret(&self, context: &Context) -> ExpResult {
        println!("*cries*");
        Ok(())
    }
}

pub struct MakeSandwich {
    pub ingredients: Vec<String>,
}

impl MakeSandwich {
    pub fn new() -> Self {
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

pub struct EatSandwich {
}

impl EatSandwich {
    pub fn new() -> Self {
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

    struct AlwaysSuccess;
    impl Expression for AlwaysSuccess {
        fn interpret(&self, context: &Context) -> ExpResult {
            Ok(())
        }
    }

    struct AlwaysFail;
    impl Expression for AlwaysFail {
        fn interpret(&self, context: &Context) -> ExpResult {
            Err(Box::new("This expression always fails"))
        }
    }

    #[test]
    fn should_successfully_interpret_and() {
        // given
        let context: Context = RefCell::new(HashMap::new());
        let and_expression = And::new(vec!(
            Box::new(AlwaysSuccess),
            Box::new(AlwaysSuccess),
        ));

        // when
        let result = and_expression.interpret(&context);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn and_should_fail() {
        // given
        let context: Context = RefCell::new(HashMap::new());
        let and_expression = And::new(vec!(
            Box::new(AlwaysSuccess),
            Box::new(AlwaysFail),
        ));

        // when
        let result = and_expression.interpret(&context);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn or_should_succeed() {
        // given
        let context: Context = RefCell::new(HashMap::new());
        let or_expression = Or::new(vec!(
            Box::new(AlwaysFail),
            Box::new(AlwaysFail),
            Box::new(AlwaysFail),
            Box::new(AlwaysSuccess),
        ));

        // when
        let result = or_expression.interpret(&context);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn or_should_fail() {
        // given
        let context: Context = RefCell::new(HashMap::new());
        let or_expression = Or::new(vec!(
            Box::new(AlwaysFail),
            Box::new(AlwaysFail),
            Box::new(AlwaysFail),
        ));

        // when
        let result = or_expression.interpret(&context);

        // then
        assert!(result.is_err());
    }
}

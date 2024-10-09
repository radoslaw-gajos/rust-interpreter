use interpreter::*;
use interpreter::Context;
use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
        // given
        let ingredients = vec!("cheese".to_string());
        let context: Context = RefCell::new(HashMap::new());
        context.borrow_mut().insert("cheese".to_string(), Box::new(()));
        let mut make_sandwich = MakeSandwich::new();
        make_sandwich.ingredients = ingredients;
        let and_expression = And::new(vec!(
            Box::new(make_sandwich),
            Box::new(EatSandwich::new()),
        ));
        let or_expression = Or::new(vec!(
            Box::new(EatSandwich::new()),
            Box::new(Cry::new()),
        ));
        let big_expression = And::new(vec!(
            Box::new(and_expression),
            Box::new(or_expression),
        ));

        // when
        let result = big_expression.interpret(&context);

        // then
        assert!(result.is_ok());
}

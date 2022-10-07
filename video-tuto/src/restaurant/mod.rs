const assigned : String = "assigned".to_string();

const in_progress : String = "in_progress".to_string();

const done : String = "done".to_string();

const failed : String = "failed".to_string();

const canceled : String = "canceled".to_string();


mod pizza_order {
  pub struct Pizza {
    pub dough: String,
    pub cheese: String,
    pub topping: String,
  }

  impl Pizza {
    pub fn lunch(topping: &str) -> Pizza {
      Pizza {
        dough: String::from("regular dough"),
        cheese: String::from("mozzarella"),
        topping: String::from(topping),
      }
    }
  }
}

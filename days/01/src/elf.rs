use crate::food::Food;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    food_items: Vec<Food>,
}

impl Elf {
    pub fn new() -> Elf {
        Elf { food_items: vec![] }
    }

    pub fn add_food(&mut self, food: Food) {
        self.food_items.push(food);
    }

    pub fn total_calories(&self) -> i32 {
        self.food_items.iter().map(|food| food.calories).sum()
    }

    pub fn food_items(&self) -> &Vec<Food> {
        &self.food_items
    }
}

impl From<Vec<Food>> for Elf {
    fn from(food_items: Vec<Food>) -> Self {
        Elf { food_items }
    }
}

impl From<Vec<i32>> for Elf {
    fn from(calories: Vec<i32>) -> Self {
        let food_items = calories
            .into_iter()
            .map(|calories| Food::new(calories))
            .collect();
        Elf { food_items }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_elf() {
        let elf = Elf::new();
        assert_eq!(elf.food_items, vec![]);
    }

    #[test]
    fn test_add_food() {
        let mut elf = Elf::new();
        let food = Food::new(100);
        elf.add_food(food);
        assert_eq!(elf.food_items, vec![Food { calories: 100 }]);
    }

    #[test]
    fn test_total_calories() {
        let mut elf = Elf::new();
        let food = Food::new(100);
        elf.add_food(food);
        let food = Food::new(200);
        elf.add_food(food);
        assert_eq!(elf.total_calories(), 300);
    }
}

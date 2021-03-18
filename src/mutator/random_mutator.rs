use super::api::Mutator;
use rand::Rng;

pub struct RandomMutator {
    pub min_mutations: u32,
    pub max_mutations: u32
}

impl RandomMutator {

    fn swap_char(original_string: &String) -> String {
        if original_string.len() == 0 {
            return original_string.clone();
        }
        let mut rng = rand::thread_rng();
        let mut mutated_string = String::with_capacity(original_string.len());
        let new_character = rand::random::<char>();
        let position = rng.gen_range(0..original_string.len());
        for (i, c) in original_string.chars().enumerate() {
            if i == position {
                mutated_string.push(new_character);
            }
            else {
                mutated_string.push(c);
            }            
        }
        return mutated_string
    }

    fn insert_random_character(original_string: &String) -> String {
        let new_character = rand::random::<char>();
        let mut mutated_string = String::with_capacity(original_string.len()+1);
        if original_string.len() == 0 {
            mutated_string.push(new_character);
            return mutated_string;
        }
        let inserted_position = rand::thread_rng().gen_range(0..original_string.len());
        for (i, c) in original_string.chars().enumerate() {
            if i == inserted_position {
                mutated_string.push(new_character);
            }
            mutated_string.push(c);
        }
        return mutated_string
    }

    fn delete_random_character(original_string: &String) -> String {
        if original_string.len() == 0 {
            return original_string.clone();
        }
        let mut rng = rand::thread_rng();
        let mut mutated_string = String::with_capacity(original_string.len()-1);
        let deleted_position = rng.gen_range(0..original_string.len());
        for (i, c) in original_string.chars().enumerate() {
            if i != deleted_position {
                mutated_string.push(c);
            }         
        }
        return mutated_string
    }

    fn choose_random_mutation() -> fn(&String) -> String {
        let mutation_list = [RandomMutator::swap_char, RandomMutator::delete_random_character, RandomMutator::insert_random_character];

        let mutation_index = rand::thread_rng().gen_range(0..mutation_list.len());
        return mutation_list[mutation_index];
    }
}

impl Mutator for RandomMutator {
    fn mutate(&self, target: &String) -> String {
        let number_of_mutations = rand::thread_rng().gen_range(self.min_mutations..=self.max_mutations);
        let mut mutated_target = target.clone();
        for _ in 1..=number_of_mutations {
            let mutation = RandomMutator::choose_random_mutation();
            mutated_target = mutation(&mutated_target);
        }
        return mutated_target;
    }
}


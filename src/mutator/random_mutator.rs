use super::api::Mutator;
use rand::Rng;
use rand::distributions::Uniform;

pub struct RandomMutator {
    pub min_mutations: u32,
    pub max_mutations: u32,
    pub char_start : u8,
    pub char_range : u8,
}
const MIN_MUTATIONS: u32 = 1;
const MAX_MUTATIONS: u32 = 10;
const CHAR_START : u8 = 32;
const CHAR_RANGE : u8 = 64;

impl RandomMutator {

    pub fn default() -> RandomMutator {
        return RandomMutator {
            min_mutations: MIN_MUTATIONS,
            max_mutations: MAX_MUTATIONS,
            char_start: CHAR_START,
            char_range: CHAR_RANGE,
        }
    }

    fn swap_char(&self, original_string: &String) -> String {
        let new_character = self.generate_random_char();

        if original_string.len() == 0 {
            return original_string.clone();
        }
        let mut rng = rand::thread_rng();
        let mut mutated_string = String::with_capacity(original_string.len());
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

    fn insert_random_character(&self, original_string: &String) -> String {
        let new_character = self.generate_random_char();
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

    fn delete_random_character(&self, original_string: &String) -> String {
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

    fn choose_random_mutation(&self) -> fn(&RandomMutator, &String) -> String {
        let mutation_list = [RandomMutator::swap_char, RandomMutator::delete_random_character, RandomMutator::insert_random_character];

        let mutation_index = rand::thread_rng().gen_range(0..mutation_list.len());
        return mutation_list[mutation_index];
    }

    fn generate_random_char(&self) -> char {
        let mut rng = rand::thread_rng();
        let character_distribution = Uniform::from(self.char_start..=self.char_start + self.char_range);
        return char::from(rng.sample(character_distribution));
    }
    
}

impl Mutator for RandomMutator {
    fn mutate(&self, target: &String) -> String {
        let number_of_mutations = rand::thread_rng().gen_range(self.min_mutations..=self.max_mutations);
        let mut mutated_target = target.clone();
        for _ in 1..=number_of_mutations {
            let mutation = self.choose_random_mutation();
            mutated_target = mutation(&self, &mutated_target);
        }
        return mutated_target;
    }
}


#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub class: String,
    pub payment_history: Vec<u32>,
}

#[contract]
pub struct SchoolManagementSystem;

#[contractimpl]
impl SchoolManagementSystem {
    pub fn create_student(env: Env, id: u32, name: String, class: String) {
        let student = Student {
            id,
            name,
            class,
            payment_history: vec![&env],
        };
        env.storage().persistent().set(&id, &student);
    }

    pub fn add_payment(env: Env, id: u32, amount: u32) {
        let mut student: Student = env.storage().persistent().get(&id).unwrap();
        student.payment_history.push_back(amount);
        env.storage().persistent().set(&id, &student);
    }

    pub fn update_student_class(env: Env, id: u32, new_class: String) {
        let mut student: Student = env.storage().persistent().get(&id).expect("Student not found");
        student.class = new_class;
        env.storage().persistent().set(&id, &student);
    }

    pub fn get_payment_history(env: Env, id: u32) -> Vec<u32> {
        let student: Student = env.storage().persistent().get(&id).expect("Student not found");
        student.payment_history
    }

    pub fn remove_student(env: Env, id: u32) {
        if !env.storage().persistent().has(&id) {
            panic!("Student not found");
        }
        env.storage().persistent().remove(&id);
    }
    
    pub fn has_student(env: Env, id: u32) -> bool {
        env.storage().persistent().has(&id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_school_management_system() {
        let env = Env::default();
        
        // This universally handles fallback checking for your test environment configuration
        let contract_id = env.register_contract(None, SchoolManagementSystem);
        let client = SchoolManagementSystemClient::new(&env, &contract_id);

        let student_id = 101;
        let name = String::from_str(&env, "Alice Smith");
        let initial_class = String::from_str(&env, "Grade 10");

        client.create_student(&student_id, &name, &initial_class);

        let new_class = String::from_str(&env, "Grade 11");
        client.update_student_class(&student_id, &new_class);

        client.add_payment(&student_id, &500);
        client.add_payment(&student_id, &600);
        let history = client.get_payment_history(&student_id);
        assert_eq!(history.get(0).unwrap(), 500);
        assert_eq!(history.get(1).unwrap(), 600);

        assert!(client.has_student(&student_id));
        client.remove_student(&student_id);
        assert!(!client.has_student(&student_id));
    }
}

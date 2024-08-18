#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, Symbol, Map};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Status {
    Active,
    Completed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Event {
    pub event_id: u32,
    pub name: Symbol,
    pub description: Symbol,
    pub start_date: u64,
    pub end_date: u64,
    pub status: Status,
}

#[contract]
pub struct EventsContract;

#[contractimpl]
impl EventsContract {
    pub fn create_event(env: Env, name: Symbol, description: Symbol, start_date: u64, end_date: u64) -> u32 {
        let mut events: Map<u32, Event> = env.storage().persistent().get(&Symbol::new(&env, "events")).unwrap_or(Map::new(&env));
        let event_counter: u32 = env.storage().persistent().get(&Symbol::new(&env, "event_counter")).unwrap_or(0) + 1;

        let new_event = Event {
            event_id: event_counter,
            name,
            description,
            start_date,
            end_date,
            status: Status::Active,
        };

        events.set(event_counter, new_event.clone());
        env.storage().persistent().set(&Symbol::new(&env, "events"), &events);
        env.storage().persistent().set(&Symbol::new(&env, "event_counter"), &event_counter);

        event_counter
    }

    pub fn read_event(env: Env, event_id: u32) -> Option<Event> {
        let events: Map<u32, Event> = env.storage().persistent().get(&Symbol::new(&env, "events")).unwrap_or(Map::new(&env));
        events.get(event_id)
    }

    pub fn update_event(env: Env, event_id: u32, name: Symbol, description: Symbol, start_date: u64, end_date: u64, status: Status) {
        let mut events: Map<u32, Event> = env.storage().persistent().get(&Symbol::new(&env, "events")).unwrap_or(Map::new(&env));
        if let Some(mut event) = events.get(event_id) {
            event.name = name;
            event.description = description;
            event.start_date = start_date;
            event.end_date = end_date;
            event.status = status;
            events.set(event_id, event);
            env.storage().persistent().set(&Symbol::new(&env, "events"), &events);
        }
    }

    pub fn delete_event(env: Env, event_id: u32) {
        let mut events: Map<u32, Event> = env.storage().persistent().get(&Symbol::new(&env, "events")).unwrap_or(Map::new(&env));
        events.remove(event_id);
        env.storage().persistent().set(&Symbol::new(&env, "events"), &events);
    }

    pub fn list_events(env: Env) -> Vec<Event> {
        let events: Map<u32, Event> = env.storage().persistent().get(&Symbol::new(&env, "events")).unwrap_or(Map::new(&env));
        let mut event_list: Vec<Event> = Vec::new(&env);

        for event in events.values() {
            event_list.push_back(event);
        }

        event_list
    }

    pub fn get_event_count(env: Env) -> u32 {
        env.storage().persistent().get(&Symbol::new(&env, "event_counter")).unwrap_or(0)
    }

    pub fn get_event_by_index(env: Env, index: u32) -> Option<Event> {
        let event_counter: u32 = env.storage().persistent().get(&Symbol::new(&env, "event_counter")).unwrap_or(0);
        if index < event_counter {
            let events: Map<u32, Event> = env.storage().persistent().get(&Symbol::new(&env, "events")).unwrap_or(Map::new(&env));
            events.get(index + 1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Symbol};

    #[test]
    fn test_create_event() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EventsContract);
        let client = EventsContractClient::new(&env, &contract_id);

        let event_id = client.create_event(
            &Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
            &123456u64, // Updated: Pass references to u64
            &789012u64, // Updated: Pass references to u64
        );
        assert_eq!(event_id, 1);
    }

    #[test]
    fn test_read_event() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EventsContract);
        let client = EventsContractClient::new(&env, &contract_id);

        client.create_event(
            &Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
            &123456u64, // Updated: Pass references to u64
            &789012u64, // Updated: Pass references to u64
        );

        let event = client.read_event(&1u32); // Updated: Pass reference to u32
        assert_eq!(
            event,
            Some(Event {
                event_id: 1,
                name: Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
                description: Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
                start_date: 123456,
                end_date: 789012,
                status: Status::Active,
            })
        );
    }

    #[test]
	fn test_update_event() {
		let env = Env::default();
		let contract_id = env.register_contract(None, EventsContract);
		let client = EventsContractClient::new(&env, &contract_id);

		client.create_event(
			&Symbol::new(&env, "Event1"), // Ensure valid Symbol
			&Symbol::new(&env, "Description1"), // Ensure valid Symbol
			&123456u64, // Pass references to u64
			&789012u64, // Pass references to u64
			);

		// Ensure the updated symbols are valid (e.g., alphanumeric and within length limits)
		client.update_event(
			&1u32, // Pass reference to u32
			&Symbol::new(&env, "UpdatedEvent"), // Valid Symbol with no spaces and within length limits
			&Symbol::new(&env, "UpdatedDesc"), // Valid Symbol with no spaces and within length limits
			&654321u64, // Pass references to u64
			&210987u64, // Pass references to u64
			&Status::Completed, // Pass reference to Status
			);

		let updated_event = client.read_event(&1u32); // Pass reference to u32
		assert_eq!(
			updated_event,
			Some(Event {
				event_id: 1,
				name: Symbol::new(&env, "UpdatedEvent"), // Ensure valid Symbol
				description: Symbol::new(&env, "UpdatedDesc"), // Ensure valid Symbol
				start_date: 654321,
				end_date: 210987,
				status: Status::Completed,
			})
		);
	}

    #[test]
    fn test_delete_event() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EventsContract);
        let client = EventsContractClient::new(&env, &contract_id);

        client.create_event(
            &Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
            &123456u64, // Updated: Pass references to u64
            &789012u64, // Updated: Pass references to u64
        );
        client.delete_event(&1u32); // Updated: Pass reference to u32

        let event = client.read_event(&1u32); // Updated: Pass reference to u32
        assert_eq!(event, None);
    }

    #[test]
    fn test_list_events() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EventsContract);
        let client = EventsContractClient::new(&env, &contract_id);

        client.create_event(
            &Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
            &123456u64, // Updated: Pass references to u64
            &789012u64, // Updated: Pass references to u64
        );
        client.create_event(
            &Symbol::new(&env, "Event2"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description2"), // Updated: Ensure valid Symbol
            &234567u64, // Updated: Pass references to u64
            &890123u64, // Updated: Pass references to u64
        );

        let events = client.list_events();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_get_event_count() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EventsContract);
        let client = EventsContractClient::new(&env, &contract_id);

        client.create_event(
            &Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
            &123456u64, // Updated: Pass references to u64
            &789012u64, // Updated: Pass references to u64
        );
        client.create_event(
            &Symbol::new(&env, "Event2"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description2"), // Updated: Ensure valid Symbol
            &234567u64, // Updated: Pass references to u64
            &890123u64, // Updated: Pass references to u64
        );

        let event_count = client.get_event_count();
        assert_eq!(event_count, 2);
    }

    #[test]
    fn test_get_event_by_index() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EventsContract);
        let client = EventsContractClient::new(&env, &contract_id);

        client.create_event(
            &Symbol::new(&env, "Event1"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description1"), // Updated: Ensure valid Symbol
            &123456u64, // Updated: Pass references to u64
            &789012u64, // Updated: Pass references to u64
        );
        client.create_event(
            &Symbol::new(&env, "Event2"), // Updated: Ensure valid Symbol
            &Symbol::new(&env, "Description2"), // Updated: Ensure valid Symbol
            &234567u64, // Updated: Pass references to u64
            &890123u64, // Updated: Pass references to u64
        );

        let event = client.get_event_by_index(&1u32); // Updated: Pass reference to u32
        assert_eq!(
            event,
            Some(Event {
                event_id: 2,
                name: Symbol::new(&env, "Event2"), // Updated: Ensure valid Symbol
                description: Symbol::new(&env, "Description2"), // Updated: Ensure valid Symbol
                start_date: 234567,
                end_date: 890123,
                status: Status::Active,
            })
        );
    }
}


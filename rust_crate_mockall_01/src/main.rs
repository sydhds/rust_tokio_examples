use function_name::named;
// use mockall::automock;

// By using this line, we would require mockall to be a dependency in Cargo.toml
// , but we want to use it as a test dependency
// #[automock]
#[cfg_attr(test, mockall::automock)]
trait Controller {
    // Note: #[automock] will generate a MockController struct
    //       create: let mut mock = MockController::new();

    fn save_event(&mut self, event_id: u32);
    fn get_events(&self) -> Vec<u32>;
    // Commented as it will make the trait not object safe
    // fn get_events_it(&self) -> impl Iterator<Item = u32>;
}

// Regular impl for trait Controller
struct DiskBasedController {}

impl Controller for DiskBasedController {
    // Note: For this example, we do not implement save_event && get_events,
    //       but it should save an event to disk & read from disk to return all events,
    //       and we assume that the implementation is quite complex ...

    #[named]
    fn save_event(&mut self, _event_id: u32) {
        println!("Stub implementations: {}", function_name!());
    }

    #[named]
    fn get_events(&self) -> Vec<u32> {
        println!("Stub implementations: {}", function_name!());
        vec![]
    }

    // Not supported by mockall 0.13.1
    // #[named]
    // fn get_events_it(&self) -> impl Iterator<Item = u32> {
    //     println!("Stub implementations: {}", function_name!());
    //     std::iter::empty()
    // }
}

struct VM {
    controller: Box<dyn Controller>,
}

impl VM {
    fn save_event(&mut self, event_id: u32) {
        self.controller.save_event(event_id);
    }

    fn get_events(&self) -> Vec<u32> {
        self.controller.get_events()
    }
}

fn main() {
    let mut vm = VM {
        controller: Box::new(DiskBasedController {}),
    };

    vm.save_event(3);
    vm.save_event(42);

    println!("{:?}", vm.controller.get_events());
}

#[cfg(test)]
mod test {
    use super::*;
    // std
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_controller() {
        // Store events in memory
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone_1 = events.clone();
        let events_clone_2 = events.clone();

        let mut mock = MockController::new();
        mock.expect_save_event()
            .withf(move |event_id| {
                // Store events in memory
                println!("Mock: Saving events (id: {})", event_id);
                events_clone_1
                    .lock()
                    .unwrap()
                    .push(*event_id);
                true
            })// Use withhf expectations to store events
            .times(2) // Enforce that save_events will be called twice
            .return_const(()) // Enforce the return value of MockController::save_event
        ;
        mock.expect_get_events().returning(move || {
            println!("Mock: Fetching events...");
            events_clone_2.lock().unwrap().clone()
        });

        let mut vm = VM {
            controller: Box::new(mock),
        };

        vm.save_event(1);
        vm.save_event(12);
        assert_eq!(vm.get_events(), vec![1, 12]);
    }
}

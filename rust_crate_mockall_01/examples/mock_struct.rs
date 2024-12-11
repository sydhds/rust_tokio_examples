use rust_crate_mockall_lib::HDBasedController;

#[derive(Debug, Clone)]
struct VM2 {
    controller: HDBasedController,
}

impl VM2 {
    fn save_event(&mut self, event_id: u32) {
        self.controller.save_event(event_id);
    }

    fn get_events(&self) -> Vec<u32> {
        self.controller.get_events()
    }
}

fn main() {
    println!("Hello, world!");

    let vm = VM2 {
        // Note: The mock struct should implement a new impl
        //       will not work with HDBaseController { inner: 1 }
        controller: HDBasedController::new(42),
    };

    let events = vm.controller.get_events();
    dbg!(events);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_save_event() {
        let mut controller = HDBasedController::faux();
        faux::when!(controller.save_event).then(|_| ());
        faux::when!(controller.get_events).then(|_| vec![1]);
        faux::when!(controller.get_events_it).then(|_| std::iter::once(1));
        let mut vm = VM2 { controller };
        vm.save_event(1);
        let events = vm.get_events();
        assert_eq!(events, vec![1]);

        let events_it = vm.controller.get_events_it();
        assert_eq!(events, vec![1]);
    }
}

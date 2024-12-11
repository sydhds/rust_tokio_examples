use function_name::named;

#[cfg_attr(any(test, feature = "faux"), faux::create)]
#[derive(Debug, Clone)]
pub struct HDBasedController {
    pub inner: u8,
}

#[cfg_attr(any(test, feature = "faux"), faux::methods)]
impl HDBasedController {
    pub fn new(inner: u8) -> Self {
        Self { inner }
    }

    #[named]
    pub fn save_event(&mut self, _event_id: u32) {
        self.save_event_inner(_event_id)
    }

    #[named]
    fn save_event_inner(&mut self, _event_id: u32) {
        println!("Stub implementations: {}", function_name!());
    }

    #[named]
    pub fn get_events(&self) -> Vec<u32> {
        println!("Stub implementations: {}", function_name!());
        vec![]
    }

    // Not supported by Faux 0.1.11
    /*
    #[named]
    pub fn get_events_it(&self) -> impl Iterator<Item = u32> {
        println!("Stub implementations: {}", function_name!());
        std::iter::empty()
    }
    */
}

#[derive(Debug, Clone)]
pub struct HDBasedController2 {
    pub inner: u8,
}

impl HDBasedController2 {
    pub fn new(inner: u8) -> Self {
        Self { inner }
    }

    #[named]
    pub fn save_event(&mut self, _event_id: u32) {
        self.save_event_inner(_event_id)
    }

    #[named]
    fn save_event_inner(&mut self, _event_id: u32) {
        println!("Stub implementations: {}", function_name!());
    }

    #[named]
    pub fn get_events(&self) -> Vec<u32> {
        println!("Stub implementations: {}", function_name!());
        vec![]
    }

    #[named]
    pub fn get_events_it(&self) -> impl Iterator<Item = u32> {
        println!("Stub implementations: {}", function_name!());
        std::iter::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_controller_save_event() {
        // This makes little sense here to test a mock here
        // but this is only for the example here

        let mut controller = HDBasedController::faux();
        // Note: cannot mock a non pub function
        // faux::when!(controller.save_event_inner).then(|_| ());
        faux::when!(controller.save_event).then(|_| ());
        faux::when!(controller.get_events).then(|_| vec![1]);
        controller.save_event(1);
        let events = controller.get_events();
        println!("events: {:?}", events);
    }

    #[ignore]
    #[test]
    fn test_controller_save_event_2() {
        let mut controller = HDBasedController::faux();
        let mut controller_2: *mut HDBasedController = &mut controller;

        // Note: try to call a private method (`save_event_inner`) when calling public method
        //       `save_event` but the test will panic
        unsafe {
            faux::when!(controller_2.as_mut().unwrap().save_event)
                .then_unchecked(|argument_0| controller.save_event_inner(argument_0));
        }

        faux::when!(controller.get_events).then(|_| vec![1]);
        controller.save_event(1);
        let events = controller.get_events();
        println!("events: {:?}", events);
    }
}

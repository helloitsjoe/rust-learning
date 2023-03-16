use super::super::Input;

pub struct MBTA {}

impl MBTA {
    pub fn new() -> MBTA {
        MBTA {}
    }

    pub fn start(self, input: &mut Input) {
        println!("Hi");
        self.handle_input(input);
    }

    pub fn handle_input(self, input: &mut Input) {
        println!("Please enter a route");
        let binding = input.get_input().to_lowercase();
        let route = binding.as_str();
        println!("Route: {:?}", route);
        // Request route
    }
}

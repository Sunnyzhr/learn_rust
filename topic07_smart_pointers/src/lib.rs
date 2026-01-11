// Exercise: Interior Mutability
//
// Goal: Create a `MockMessenger` that records messages sent to it.
// Since `send` takes &self (immutable), we need RefCell to store the messages.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct MockMessenger {
    pub sent_messages: std::cell::RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: std::cell::RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        
        // Simulating usage
        mock_messenger.send("Hello");
        mock_messenger.send("World");

        // Check if messages were recorded
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
    }
}

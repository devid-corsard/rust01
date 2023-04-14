pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max:   usize,
}

impl<'a, T> LimitTracker<'a, T> 
where 
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0 , max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value * 100 / self.max;

//        if percentage_of_max >= 100 {
//            self.messenger.send("Error! You are over your quota!");
//        } else if percentage_of_max >=90 {
//            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
//        } else if percentage_of_max >=75 {
//            self.messenger.send("Warning: You've used up over 75% of your quota!");
//        }
        match percentage_of_max {
            100.. => self.messenger.send("Error! You are over your quota!"),
            90..=99 => self.messenger.send("Urgent warning: You've used up over 90% of your quota!"),
            75..=89 => self.messenger.send("Warning: You've used up over 75% of your quota!"),
            _ => (),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messeges: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messeges: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //self.sent_messeges.borrow_mut().push(String::from(message));
            let mut one_borrow = self.sent_messeges.borrow_mut();
            let mut sec_borrow = self.sent_messeges.borrow_mut();

            one_borrow.push(String::from(message));
            sec_borrow.push(String::from(message));

        }
    }

    #[test]
    fn sends_over_75_percent() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        
        limit_tracker.set_value(75);

        assert_eq!(mock_messenger.sent_messeges.borrow().len(), 1);

    }
}
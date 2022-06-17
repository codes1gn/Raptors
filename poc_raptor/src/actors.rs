// LICENSE PLACEHOLDER
//
use crate::messages;

// placehold for actors

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::{thread, time};

    // test visibility
    #[test]
    fn create_dummy_workload_test() {
        let load = messages::DummyWorkload::new(16);
        assert_eq!(load.payload(), 16 as usize);
    }

    #[test]
    fn worklaod_mock_run_test() {
        let load = messages::DummyWorkload::new(16);
        let now = time::Instant::now();
    }
}

use std::marker::PhantomPinned;
use std::pin::pin;
#[derive(Default)]
struct AddrTracker {
    prev_addr: Option<usize>,
    _pin: PhantomPinned,
}
impl AddrTracker {
    // If we haven't checked the addr of self yet, store the current
    // address. If we have, confirm that the current address is the same
    // as it was last time, or else panic.
    fn check_for_move(&mut self) {
        let current_addr = self as *mut Self as usize;
        match self.prev_addr {
            None => {
                self.prev_addr = Some(current_addr);
            },
            Some(prev_addr) => assert_eq!(prev_addr, current_addr),
        }
    }
}


fn main() {
    // Create a tracker and store the initial address
    let mut tracker = AddrTracker::default();
    let mut ptr_to_pinned_tracker = pin!(tracker);
    ptr_to_pinned_tracker.as_mut().check_for_move()
    // Here we shadow the variable. This carries a semantic move, and may therefore also
    // come with a mechanical memory *move*

    // may panic

}
use core::fmt::Debug;
use std::time::Instant;

///
/// Subscribe callback/closure type, allocate on heap
///
type StateCallback<State> = Box<dyn Fn(&State)>;

///
/// Allow subscribe to `StateService`
///
pub trait Subscriber<State>
where
    State: Clone + PartialOrd + Debug,
{
    fn subscribe(
        service: &mut StateService<State>,
        callback: StateCallback<State>,
    ) -> StateServiceSubscription;
}

///
/// Allow unsubscribe from `StateService`
///
pub trait Subscription<State>
where
    State: Clone + PartialOrd + Debug,
{
    fn unsubscribe(&self, service: &mut StateService<State>);
}

///
/// Subcription represents the particular `(Time, callback)` pair
///
pub struct StateServiceSubscription {
    callback_pair_key: Instant,
}

//
impl StateServiceSubscription {
    pub fn new(callback_pair_key: Instant) -> Self {
        StateServiceSubscription { callback_pair_key }
    }
}

impl<State> Subscription<State> for StateServiceSubscription
where
    State: Clone + PartialOrd + Debug,
{
    fn unsubscribe(&self, service: &mut StateService<State>) {
        service.remove_subscriber(self.callback_pair_key);
    }
}

///
/// StateService
///
pub struct StateService<State>
where
    State: Clone + PartialOrd + Debug,
{
    latest_state: State,
    subscribers: Vec<(Instant, StateCallback<State>)>,
}

impl<State> StateService<State>
where
    State: Clone + PartialOrd + Debug,
{
    //
    pub fn new(init_state: State) -> Self {
        StateService {
            latest_state: init_state,
            subscribers: Vec::new(),
        }
    }

    //
    pub fn subscribe(service: &mut StateService<State>, callback: StateCallback<State>) -> StateServiceSubscription {
        service.add_subscriber(callback)
    }

    //
    pub(crate) fn add_subscriber(&mut self, callback: StateCallback<State>) -> StateServiceSubscription  {
        // Emit the latest state immediately
        callback(&self.latest_state);

        // Push the callback pair
        let callback_pair_key = Instant::now();
        self.subscribers.push((callback_pair_key, callback));

        println!("\n>>> [ add_subscriber ] - subscribers:");
        for (time, callback) in self.subscribers.iter() {
            println!(">>> [ add_subscriber ] - ({time:?}, {callback:p})");
        }
        println!("\n");

        StateServiceSubscription::new(callback_pair_key)
    }

    //
    pub(crate) fn remove_subscriber(&mut self, callback_pair_key: Instant) {
        if let Some(index) = self
            .subscribers
            .iter()
            .position(|(time, _)| (*time) == callback_pair_key)
        {
            let (removed_time, removed_callback) = self.subscribers.get(index).unwrap();
            println!(">>> [ remove_subscriber ] - Remove callback at {index}: ({removed_time:?}, {removed_callback:p})" );
            let _ = self.subscribers.remove(index);
        }

        println!("\n>>> [ remove_subscriber ] - subscribers:");
        for (time, callback) in self.subscribers.iter() {
            println!(">>> [ remove_subscriber ] - ({time:?}, {callback:p})");
        }
        println!("\n");
    }

    //
    pub fn emit(&mut self, next_state: State) {
        // Save the state for new subscriber
        self.latest_state = next_state.clone();

        // Notify all subscribers
        for (_, callback) in self.subscribers.iter() {
            callback(&next_state);
        }
    }

    //
    pub fn get_latest_state(&self) -> State {
        self.latest_state.clone()
    }
}

#[cfg(test)]
mod tests {

    // use super::*;

    // #[derive(Debug, Clone, PartialEq, PartialOrd)]
    // struct ToDoItem {
    //     pub text: String,
    //     pub finished: bool,
    // }

    // #[derive(Debug, Clone, PartialEq, PartialOrd)]
    // struct ToDoListState {
    //     pub list: Vec<ToDoItem>,
    // }

    // impl ToDoListState {
    //     fn add_item(&mut self, text: String) {
    //         self.list.push(ToDoItem {
    //             text,
    //             finished: false
    //         })
    //     }
    // }

    // //
    // #[test]
    // fn create_init_state() {
    //     let init_state = ToDoListState { list: Vec::new() };
    //     let login_state_service = StateService::new(init_state.clone());
    //     println!("init_state: {init_state:?}");
    //     assert_eq!(login_state_service.get_latest_state(), init_state);
    // }

    // //
    // #[test]
    // fn subscribe_should_work() {
    //     let init_state = ToDoListState { list: Vec::new() };
    //     let mut login_state_service = StateService::new(init_state.clone());

    //     // Should emit the latest_state immediate
    //     login_state_service.subscribe(Box::new(|state| println!("Got next state: {state:?}")));

    //     // Emit new state
    //     let mut latest_state = login_state_service.get_latest_state();
    //     latest_state.add_item("Learn Polkadot".to_owned());
    //     latest_state.add_item("Learn Bitcoin".to_owned());
    //     login_state_service.emit(latest_state.clone());

    //     latest_state.add_item("Write a demo".to_owned());
    //     login_state_service.emit(latest_state);

    //     assert_eq!(login_state_service.get_latest_state(), init_state);
    // }
}

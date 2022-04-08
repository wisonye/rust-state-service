use state_service::{StateService, Subscription};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ToDoItem {
    pub text: String,
    pub finished: bool,
}

//
// Define your state struct
//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ToDoListState {
    pub list: Vec<ToDoItem>,
}

//
// Implement your state action (function)
//
impl ToDoListState {
    fn add_item(&mut self, text: String) {
        self.list.push(ToDoItem {
            text,
            finished: false,
        })
    }
}


//
//
//
fn main() {
    let init_state = ToDoListState { list: vec!(ToDoItem {
        text: "Fun demo".to_owned(),
        finished: false
    })};
    let mut todo_state_service = StateService::new(init_state.clone());

    // Should emit the latest_state immediate
    println!("\n[ Wison ] should get the last state: {init_state:?}");
    let wison_subscription = StateService::subscribe(
        &mut todo_state_service,
        Box::new(|state| println!("Wison >>> I got next state: {state:#?}")),
    );

    // Emit new state
    let mut latest_state = todo_state_service.get_latest_state();
    latest_state.add_item("Learn Polkadot".to_owned());
    latest_state.add_item("Learn Bitcoin".to_owned());
    todo_state_service.emit(latest_state.clone());

    // Should emit the latest_state immediate
    println!("\n[ Fion ] should get the last state: {latest_state:?}");
    let fion_subscription = StateService::subscribe(
        &mut todo_state_service,
        Box::new(|state| println!("Fion >>> I got next state: {state:#?}")),
    );

    latest_state.add_item("Write a demo".to_owned());
    todo_state_service.emit(latest_state.clone());

    // Unsubscribe
    fion_subscription.unsubscribe(&mut todo_state_service);

    latest_state.add_item("Final".to_owned());
    todo_state_service.emit(latest_state.clone());

    // Unsubscribe
    wison_subscription.unsubscribe(&mut todo_state_service);

    latest_state.add_item("No one should see that".to_owned());
    todo_state_service.emit(latest_state.clone());
    todo_state_service.emit(latest_state);
}

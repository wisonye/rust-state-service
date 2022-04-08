# Rust version `State Service`

Port my `TypeScript` version `StateService`  to `Rust`.

### How to use it

- Define your state struct and implement your action functions.

    ```rust
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
    ```

    </br>

- Create your state service instance with the init state

    ```rust
    let init_state = ToDoListState { list: vec!(ToDoItem {
        text: "Fun demo".to_owned(),
        finished: false
    })};
    let mut todo_state_service = StateService::new(init_state.clone());
    ```

    </br>

- Subscribe to the state service

    ```rust
    let my_subscription = StateService::subscribe(
        // Your state service instance mut ref here
        &mut todo_state_service,

        // Callback closure/function here
        // Make sure to use `Box::new()` to wrap your callback, then it
        // allocates on the heap.
        Box::new(|state| println!("My subscription >>> I got next state: {state:#?}")),
    );
    ```

    </br>

- Emit new state at any given time

    ```rust
    // Emit new state
    let mut latest_state = todo_state_service.get_latest_state();
    latest_state.add_item("Learn Polkadot".to_owned());
    latest_state.add_item("Learn Bitcoin".to_owned());
    todo_state_service.emit(latest_state.clone());
    ```

    All callback closures (allocated on the heap) will receive the new emitted
    state immediately.

    </br>

- Unsubscribe when you done with it

    ```rust
    // Unsubscribe
    my_subscription.unsubscribe(&mut todo_state_service);
    ```

    </br>


### How to run the `TodoList` example

```bash
cargo watch -c --exec "run --example to_do_list"
```

</br>


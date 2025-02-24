use std::collections::HashMap;
use std::hash::Hash;

/// A callback function that returns a boolean.
pub type Callback = Box<dyn Fn() -> bool>;

/// A finite state machine implementation.
pub struct FSM<S: Eq + Hash + Clone> {
    /// The current state of the FSM.
    pub current: S,
    /// The list of valid states.
    states: Vec<S>,
    /// A mapping from (from_state, to_state, transition_name) transitions to their associated callbacks.
    transitions: HashMap<(S, S, String), Callback>,
}

impl<S: Eq + Hash + Clone> FSM<S> {
    /// Creates a new FSM instance with an initial state and a list of states.
    ///
    /// # Panics
    ///
    /// Panics if the initial state is not in the list of valid states.
    pub fn new(initial_state: S, states: Vec<S>) -> Self {
        if !states.contains(&initial_state) {
            panic!("Initial state must be one of the provided states");
        }
        FSM {
            current: initial_state,
            states,
            transitions: HashMap::new(),
        }
    }

    /// Adds a transition from a `from` state to a `to` state with an associated callback and a transition name.
    ///
    /// The callback is executed during the transition. If it returns `true`,
    /// the transition is considered successful and the state changes.
    ///
    /// # Panics
    ///
    /// Panics if either the `from` or `to` state is not in the list of valid states.
    pub fn add_transition<F>(&mut self, from: S, to: S, transition_name: String, callback: F)
    where
        F: Fn() -> bool + 'static,
    {
        if !self.states.contains(&from) || !self.states.contains(&to) {
            panic!("Transition states must be part of the FSM states");
        }
        self.transitions
            .insert((from, to, transition_name), Box::new(callback));
    }

    /// Attempts to transition to the given state using a transition identified by `transition_name`.
    ///
    /// Executes the associated callback for the transition from the current state to `to`.
    /// If the callback returns `true`, the current state is updated to `to` and the function returns `true`.
    /// Otherwise, the state remains unchanged and the function returns `false`.
    ///
    /// If no transition is defined for the current state to `to` with the given `transition_name`, this function returns `false`.
    pub fn transition_to(&mut self, to: S, transition_name: &str) -> bool {
        let key = (
            self.current.clone(),
            to.clone(),
            transition_name.to_string(),
        );
        if let Some(callback) = self.transitions.get(&key) {
            let result = callback();
            if result {
                self.current = to;
            }
            result
        } else {
            false
        }
    }
}

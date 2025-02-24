use rsfsm::FSM;

#[test]
fn raft_election_success() {
    let follower = "Follower".to_string();
    let candidate = "Candidate".to_string();
    let leader = "Leader".to_string();
    let states = vec![follower.clone(), candidate.clone(), leader.clone()];
    let mut fsm = FSM::new(follower.clone(), states);
    
    // Follower -> Candidate transition
    fsm.add_transition(follower.clone(), candidate.clone(), "default".to_string(), || true);
    // Candidate -> Leader transition
    fsm.add_transition(candidate.clone(), leader.clone(), "default".to_string(), || true);
    
    let res = fsm.transition_to(candidate.clone(), "default");
    assert!(res);
    assert_eq!(fsm.current, candidate);
    
    let res = fsm.transition_to(leader.clone(), "default");
    assert!(res);
    assert_eq!(fsm.current, leader);
}

#[test]
fn raft_election_failed() {
    let follower = "Follower".to_string();
    let candidate = "Candidate".to_string();
    let leader = "Leader".to_string();
    let states = vec![follower.clone(), candidate.clone(), leader.clone()];
    let mut fsm = FSM::new(follower.clone(), states);
    
    // Follower -> Candidate transition
    fsm.add_transition(follower.clone(), candidate.clone(), "default".to_string(), || true);
    // Candidate -> Leader transition (failing)
    fsm.add_transition(candidate.clone(), leader.clone(), "default".to_string(), || false);
    
    let res = fsm.transition_to(candidate.clone(), "default");
    assert!(res);
    assert_eq!(fsm.current, candidate);
    
    let res = fsm.transition_to(leader.clone(), "default");
    assert!(!res);
    assert_eq!(fsm.current, candidate);
}

#[test]
fn leader_becomes_follower_on_higher_state() {
    let follower = "Follower".to_string();
    let candidate = "Candidate".to_string();
    let leader = "Leader".to_string();
    let states = vec![follower.clone(), candidate.clone(), leader.clone()];
    // Initialize FSM with leader as current state.
    let mut fsm = FSM::new(leader.clone(), states);
    
    // Transition: Leader -> Follower due to receiving a higher state.
    fsm.add_transition(leader.clone(), follower.clone(), "step_down".to_string(), || true);
    
    let res = fsm.transition_to(follower.clone(), "step_down");
    assert!(res);
    assert_eq!(fsm.current, follower);
}

#[test]
fn follower_becomes_candidate_due_to_timeout() {
    let follower = "Follower".to_string();
    let candidate = "Candidate".to_string();
    let leader = "Leader".to_string();
    let states = vec![follower.clone(), candidate.clone(), leader.clone()];
    // Initialize FSM with follower as current state.
    let mut fsm = FSM::new(follower.clone(), states);
    
    // Transition: Follower -> Candidate due to timeout (missing heartbeat).
    fsm.add_transition(follower.clone(), candidate.clone(), "timeout".to_string(), || true);
    
    let res = fsm.transition_to(candidate.clone(), "timeout");
    assert!(res);
    assert_eq!(fsm.current, candidate);
}
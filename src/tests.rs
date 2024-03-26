use super::*;

#[test]
fn should_elect_new_leader_on_timeout() {
    let node_a = Node::new(Id(0));
    let node_b = Node::new(Id(1));

    easy_connect!(node_a.clone(), node_b.clone());

    assert!(node_a.borrow_mut().on_timeout().is_ok());

    assert_eq!(node_a.borrow().role, Role::Leader);
    assert_eq!(node_b.borrow().current_leader.unwrap(), Id(0));
    assert_eq!(node_b.borrow().role, Role::Follower);
}

#[test]
fn should_not_re_elect_if_already_elected_for_term() {
    let node_a = Node::new(Id(0));
    let node_b = Node::new(Id(1));
    let node_c = Node::new(Id(2));

    easy_connect!(node_a.clone(), node_b.clone(), node_c.clone());

    node_a.borrow_mut().role = Role::Leader;
    node_b.borrow_mut().timeout_state = true;
    node_c.borrow_mut().timeout_state = true;
    assert!(node_b.borrow_mut().on_timeout().is_ok());
    assert_eq!(
        node_c.borrow_mut().on_timeout().unwrap_err(),
        Error::AlreadyElected(Id(1))
    );
    assert_eq!(node_a.borrow().current_leader.unwrap(), Id(1));
    assert_eq!(node_b.borrow().role, Role::Leader);
    assert_eq!(node_c.borrow().current_leader.unwrap(), Id(1));
}

#[test]
fn followers_should_replicate_leader_logs() {
    let node_b = Node::new(Id(1));

    node_b.borrow_mut().current_leader = Some(Id(0));

    assert!(node_b
        .borrow_mut()
        .on_rcv_message(Message::ReplicateOrHeartbeat(
            Id(0),
            LogEntry("new log entry".to_string())
        ))
        .is_ok());
    assert!(node_b.borrow().logs[0] == LogEntry("new log entry".to_string()));
}

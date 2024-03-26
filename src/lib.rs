#![allow(dead_code)]

pub use log::info;
pub use std::cell::RefCell;
pub use std::rc::Rc;

pub(crate) mod helpers;

#[cfg(test)]
mod tests;
pub(crate) mod types;

use types::{Error, Id, LogEntry, Message, Role};

use crate::helpers::send_msg;

#[derive(Debug)]
pub struct Node {
    id: Id,
    role: Role,
    term: u64,
    current_leader: Option<Id>,
    timeout_state: bool,
    edges: Vec<Rc<RefCell<Node>>>,
    logs: Vec<LogEntry>,
}

pub trait RaftNode {
    /// This is called when the node has timed-out (not heard from the leader) and it may start an
    /// election
    fn on_timeout(&mut self) -> Result<(), Error>;

    /// This is the `Message` handler when a message is received
    fn on_rcv_message(&mut self, msg: Message) -> Result<(), Error>;

    /// This is used to set up the "connections" between nodes
    fn connect_nodes(&mut self, nodes: &[Rc<RefCell<Node>>]);
}

impl Node {
    fn new(id: Id) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            id,
            role: Role::Follower,
            current_leader: None,
            term: 0,
            timeout_state: false,
            edges: vec![],
            logs: vec![],
        }))
    }
}

impl RaftNode for Node {
    fn connect_nodes(&mut self, nodes: &[Rc<RefCell<Node>>]) {
        for i in nodes {
            self.edges.push(i.clone());
        }
    }

    fn on_timeout(&mut self) -> Result<(), Error> {
        if self.role == Role::Leader {
            return Err(Error::AlreadyElected(self.id));
        } else if self.role == Role::Follower {
            // Set current role as Candidate.
            self.role = Role::Candidate;
            // Increment current term.
            self.term += 1;
            // Count the vote of the current node.
            let mut votes_received = 1;
            // [Note]: For raft majority count is > (1 / 2 * size of current validator set).
            let majority = (self.edges.len() + 1) / 2 + 1;

            // Request and count received votes from other nodes.
            for node in &self.edges {
                let node_id = node.clone().borrow_mut().id;
                let message = Message::RequestVotes(node_id, self.term);
                if send_msg(&mut node.borrow_mut(), message).is_ok() {
                    votes_received += 1;
                }
            }

            // If leader already selected in this round, return error.
            if let Some(leader) = self.current_leader {
                return Err(Error::AlreadyElected(leader));
            }

            // If votes_received >= majority count, set node as new leader,
            // inform nodes of new leader and replicate logs.
            if votes_received >= majority {
                self.role = Role::Leader;
                self.current_leader = Some(self.id);

                for node in &self.edges {
                    let log_entry = LogEntry(format!(
                        "new leader {:?} \
                        at term {:?}",
                        self.id, self.term
                    ));
                    let message = Message::ReplicateOrHeartbeat(self.id, log_entry);
                    let _ = send_msg(&mut node.borrow_mut(), message);
                }
            }
        }

        Ok(())
    }

    fn on_rcv_message(&mut self, msg: Message) -> Result<(), Error> {
        match msg {
            Message::ReplicateOrHeartbeat(sender_id, log_entry) => {
                // Reset timeout state.
                self.timeout_state = !self.timeout_state;
                // Save logs.
                self.logs.push(log_entry.clone());
                // Set current leader.
                self.current_leader = Some(sender_id);
                // Set current node to follower.
                // README Ref: [1] line 1374 : self.become_follower(m.term, m.from);
                self.role = Role::Follower;
            }
            Message::RequestVotes(sender_id, sender_term) => {
                // Handle the message.
                // README Ref: [1] line 1310: pub fn step(&mut self, m: Message) {...}
                if sender_term > self.term {
                    self.term = sender_term;
                    let can_vote = self.id.0 != INVALID_ID;
                    if can_vote {
                        // README Ref: [1] line 1469: self.log_vote_approve(&m);
                        self.logs.push(LogEntry(format!(
                            "[logterm: {:?}] cast vote for {:?} \
                            at term {:?}",
                            sender_term, sender_id, self.term
                        )));
                    } else {
                        // README Ref: [1] line 1481: self.log_vote_reject(&m);
                        self.logs.push(LogEntry(format!(
                            "[logterm: {:?}] rejected vote for {:?} \
                            at term {:?}",
                            sender_term, sender_id, self.term
                        )));
                        return Err(Error::NoQuorum);
                    }
                } else if sender_term < self.term {
                    // README Ref: [1] line 1384: We have received messages from a leader at a lower term.
                    self.logs.push(LogEntry(format!(
                        "ignored a message with lower term from {:?} with term {:?} and sender term {:?}",
                        sender_id,
                        self.term,
                        sender_term
                    )));
                    return Err(Error::InvalidTerm);
                } else if sender_term == 0 {
                    return Err(Error::Offline);
                }
            }
        }

        Ok(())
    }
}

/// A constant represents invalid id of raft.
pub const INVALID_ID: u64 = 0;

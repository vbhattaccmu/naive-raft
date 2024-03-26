/// The Id of the node
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Id(pub u64);

/// The role of the node
#[derive(Debug, PartialEq)]
pub(super) enum Role {
    Follower,
    Leader,
    Candidate,
}

/// Entry to be entered into the logs
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry(pub String);

/// Message types sent by Nodes
pub enum Message {
    /// Sent by the `Role::Leader`
    ReplicateOrHeartbeat(Id, LogEntry),
    /// Sent by the `Role::Candidate`
    RequestVotes(Id, u64),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Default error used to indicate the node is offline.
    Offline,
    /// The Candidate is not up-to-date.
    InvalidTerm,
    /// `id` has already been elected as leader.
    AlreadyElected(Id),
    /// Votes did not reach quorum, wait for next election cycle
    NoQuorum,
}

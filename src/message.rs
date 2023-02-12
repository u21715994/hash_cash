use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum Result {
    Ok,
    Err(SubscribeError)
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Challenge {
    MD5HashCash(ChallengeInput)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Hello,
    Welcome { version: u8 },
    Subscribe { name: String },
    SubscribeResult(Result),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult {
        answer: ChallengeAnswer,
        next_target: String,
    },
    ChallengeTimeout {
        message: String,
    },
    RoundSummary {
        challenge: String,
        chain: Vec<ReportedChallengeResult>,
    },
    EndOfGame {
        leader_board: PublicLeaderBoard,
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicPlayer {
    name: String,
    stream_id: String,
    score: u32,
    steps: u32,
    is_active: bool,
    total_used_time: f64,
}

#[derive(Debug, Deserialize, Serialize)]
enum ChallengeName {
    MD5HashCash,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChallengeInput {
    pub complexity: u8,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChallengeOutput {
    seed: u64,
    hashcode: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum ChallengeAnswer {
    MD5HashCash(ChallengeOutput)
}

#[derive(Debug, Deserialize, Serialize)]
enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult {
        used_time: f64,
        next_target: String
    },
    Ok {
        used_time: f64,
        next_target: String
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue,
}

#[derive(Debug, Deserialize, Serialize)]
struct PublicLeaderBoard{
    publicLeaderBoard: Vec<PublicPlayer>
}
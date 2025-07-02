use std::sync::atomic::Ordering;

use chrono::{DateTime, Utc};
use teloxide::types::{
    MessageEntity, Poll, PollOption, PollType, Seconds, Update, UpdateId, UpdateKind,
};
use teloxide_tests_macros::Changeable;

use super::{IntoUpdate, MockMessagePoll};

#[derive(Changeable, Clone)]
pub struct MockUpdatePoll {
    pub poll_id: String,
    pub question: String,
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<PollOption>,
    pub is_closed: bool,
    pub total_voter_count: u32,
    pub is_anonymous: bool,
    pub poll_type: PollType,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<Seconds>,
    pub close_date: Option<DateTime<Utc>>,
}

impl MockUpdatePoll {
    /// Creates a new easily changable poll update builder
    ///
    /// # Example
    /// ```
    /// let update = teloxide_tests::MockUpdatePoll::new()
    ///     .poll_id("123456");
    ///
    /// assert_eq!(update.poll_id, "123456");
    /// ```
    pub fn new() -> Self {
        let poll = MockMessagePoll::new();
        Self {
            poll_id: poll.poll_id,
            question: poll.question,
            question_entities: poll.question_entities,
            options: poll.options,
            is_closed: poll.is_closed,
            total_voter_count: poll.total_voter_count,
            is_anonymous: poll.is_anonymous,
            poll_type: poll.poll_type,
            allows_multiple_answers: poll.allows_multiple_answers,
            correct_option_id: poll.correct_option_id,
            explanation: poll.explanation,
            explanation_entities: poll.explanation_entities,
            open_period: poll.open_period,
            close_date: poll.close_date,
        }
    }
}

impl IntoUpdate for MockUpdatePoll {
    fn into_update(self, id: &std::sync::atomic::AtomicI32) -> Vec<Update> {
        vec![Update {
            id: UpdateId(id.fetch_add(1, Ordering::Relaxed) as u32),
            kind: UpdateKind::Poll(Poll {
                id: self.poll_id.into(),
                question: self.question,
                question_entities: self.question_entities,
                options: self.options,
                is_closed: self.is_closed,
                total_voter_count: self.total_voter_count,
                is_anonymous: self.is_anonymous,
                poll_type: self.poll_type,
                allows_multiple_answers: self.allows_multiple_answers,
                correct_option_id: self.correct_option_id,
                explanation: self.explanation,
                explanation_entities: self.explanation_entities,
                open_period: self.open_period,
                close_date: self.close_date,
            }),
        }]
    }
}

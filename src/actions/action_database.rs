use std::{cmp::Ordering, sync::Arc};

use fuzzy_matcher::skim::SkimMatcherV2;

use super::{action::Action, search_db::score_action};

pub struct ActionDatabase {
    pub actions: Vec<Arc<Action>>,
}

impl ActionDatabase {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add(&mut self, action: Action) {
        self.actions.push(Arc::from(action));
    }

    pub fn get_action_results(&self, query: &String) -> Vec<Arc<Action>> {
        let matcher = SkimMatcherV2::default();

        let mut results = self.actions.clone();

        results.sort_by(|a, b| -> Ordering {
            let a_score = score_action(a, query, &matcher);
            let b_score = score_action(b, query, &matcher);
            a_score.cmp(&b_score)
        });

        return results
    }
}

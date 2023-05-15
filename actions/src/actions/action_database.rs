use std::{cmp::Ordering, sync::Arc};

use fuzzy_matcher::skim::SkimMatcherV2;

use super::{action::Action, search_db::score_action};

#[derive(Default)]
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

    pub fn iter(&self) -> std::slice::Iter<Arc<Action>> {
        self.actions.iter()
    }

    pub fn get_action_results(&self, query: &str) -> Vec<Arc<Action>> {
        let matcher = SkimMatcherV2::default();

        let mut results = self.actions.clone();

        results.sort_by(|a, b| -> Ordering {
            let a_score = score_action(a, query, &matcher);
            let b_score = score_action(b, query, &matcher);
            b_score.cmp(&a_score)
        });

        results
    }
}


use std::sync::Arc;

use simsearch::SimSearch;

use super::action::Action;

#[derive(Default)]
pub struct ActionDatabase {
    pub actions: Vec<Arc<Action>>,
    search_engine: SimSearch<usize>,
}

impl ActionDatabase {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            search_engine: SimSearch::new(),
        }
    }

    pub fn add(&mut self, action: Action) {
        self.search_engine
            .insert(self.actions.len(), action.search_queriable().as_str());
        self.actions.push(Arc::from(action));
    }

    pub fn iter(&self) -> std::slice::Iter<Arc<Action>> {
        self.actions.iter()
    }

    pub fn get_action_results(&self, query: &str) -> Vec<Arc<Action>> {
        if query == "" {
            return self.actions.clone();
        }

        let results = self.search_engine.search(query);

        results
            .iter()
            .map(|x| self.actions.get(*x).cloned())
            .flatten()
            .collect::<Vec<Arc<Action>>>()
    }
}

use std::collections::HashMap;
#[derive(Default, Debug, Clone)]
pub struct Node {
    pub children: HashMap<char, Node>,
    pub epsilon: bool, // is it the end??
}

impl Node {
    pub fn push(&mut self, mut word: Vec<char>) {
        let c = if let Some(inner) = word.pop() {
            inner
        } else {
            self.epsilon = true;
            return;
        }; // default case

        let mut self_next = self.children.remove(&c)
            .unwrap_or(Node::default());
        self_next.push(word);

        self.children.insert(c, self_next);
    }
}

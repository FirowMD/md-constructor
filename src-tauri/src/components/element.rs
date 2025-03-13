use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MarkdownElement {
    pub element_name: String,
    pub element_type: String,
    pub labels: Vec<String>,
    pub values: Vec<String>,
    pub chosens: Vec<String>,
    pub is_collapsed: bool,
}

impl MarkdownElement {
    pub fn new(
        element_name: String,
        element_type: String,
        labels: Vec<String>,
        values: Vec<String>,
        chosens: Vec<String>,
        is_collapsed: bool,
    ) -> Self {
        Self {
            element_name,
            element_type,
            labels,
            values,
            chosens,
            is_collapsed,
        }
    }

    pub fn add_value(&mut self, value: String, label: String) {
        self.values.push(value);
        self.labels.push(label);
    }

    pub fn remove_value(&mut self, value: String) {
        if let Some(index) = self.values.iter().position(|x| x == &value) {
            self.values.remove(index);
            self.labels.remove(index);
        }
    }

    pub fn change_value(&mut self, old_value: String, new_value: String, new_label: String) {
        if let Some(index) = self.values.iter().position(|x| x == &old_value) {
            self.labels[index] = new_label;
            self.values[index] = new_value;
        }
    }

    pub fn clear_values(&mut self) {
        self.values.clear();
        self.labels.clear();
    }
}

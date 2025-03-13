use crate::components::element::MarkdownElement;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MarkdownConverter {
    raw_text: String,
    elements: Vec<MarkdownElement>,
}

impl MarkdownConverter {
    pub fn new(raw_text: String, elements: Vec<MarkdownElement>) -> Self {
        Self { raw_text, elements }
    }

    pub fn add_element(&mut self, element: MarkdownElement) {
        self.elements.push(element);
    }

    pub fn remove_element(&mut self, element: MarkdownElement) {
        if let Some(index) = self.elements.iter().position(|x| x == &element) {
            self.elements.remove(index);
        }
    }

    pub fn move_element(&mut self, old_index: usize, new_index: usize) {
        let element = self.elements.remove(old_index);
        self.elements.insert(new_index, element);
    }

    /// replaces all `{% element_name %}` with the element's value
    pub fn convert(&self) -> String {
        let mut result = self.raw_text.clone();

        for element in &self.elements {
            let pattern = format!("{}{}{}", "{% ", element.element_name, " %}");
            let chosen_values: Vec<String> = element.chosens.clone();

            let replacement = chosen_values.join(" ");
            result = result.replace(&pattern, &replacement);
        }

        result
    }

    pub fn set_raw_text(&mut self, text: String) {
        self.raw_text = text;
    }
}

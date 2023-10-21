use crate::html5::element_class::ElementClass;
use crate::html5::node::data::text::TextData;
use crate::html5::node::{Node, NodeData, NodeId, HTML_NAMESPACE};
use crate::html5::parser::{ActiveElement, Html5Parser, Scope};
use crate::html5::tokenizer::token::Token;
use std::collections::HashMap;

const ADOPTION_AGENCY_OUTER_LOOP_DEPTH: usize = 8;
const ADOPTION_AGENCY_INNER_LOOP_DEPTH: usize = 3;

pub enum InsertionPositionMode<NodeId> {
    LastChild(NodeId),
    Sibling { parent: NodeId, before: NodeId },
}

impl<'stream> Html5Parser<'stream> {
    fn current_node_id(&self) -> &NodeId {
        self.open_elements.last().unwrap_or_default()
    }

    fn current_node(&self) -> Node {
        let current_node_id = self.current_node_id().clone();
        self.document
            .get()
            .get_node_by_id(current_node_id)
            .unwrap()
            .clone()
    }

    fn get_node_id(&self, node_id: &NodeId) -> Node {
        self.document
            .get()
            .get_node_by_id(*node_id)
            .unwrap()
            .clone()
    }

    fn position_in_active_format(&self, node_id: &NodeId) -> Option<usize> {
        self.active_formatting_elements
            .iter()
            .position(|&x| x == ActiveElement::Node(*node_id))
    }

    fn position_in_open_element(&self, node_id: &NodeId) -> Option<usize> {
        self.open_elements.iter().position(|x| x == node_id)
    }

    fn find_format_element(&self, subject: &str) -> Option<(usize, NodeId)> {
        self.active_formatting_elements
            .iter()
            .enumerate()
            .rev()
            .filter(|&(_, x)| match x {
                ActiveElement::Marker => false,
                ActiveElement::Node(node_id) => self.get_node_id(node_id).name == subject,
            })
            .next()
            .map(|(i, x)| match x {
                ActiveElement::Marker => panic!("not reached"),
                ActiveElement::Node(node_id) => (i, node_id.clone()),
            })
    }

    fn find_further_block(&self, format_ele_position: usize) -> Option<(usize, NodeId)> {
        self.open_elements
            .iter()
            .enumerate()
            .skip(format_ele_position)
            .filter(|&(_, x)| self.get_node_id(x).is_special())
            .next()
            .map(|(i, x)| (i, x.clone()))
    }

    /// node_type true is for text node, false if other
    pub fn insert_helper(
        &mut self,
        node: NodeId,
        position: InsertionPositionMode<NodeId>,
        node_type: bool,
        token: Option<&Token>,
    ) {
        match position {
            InsertionPositionMode::Sibling { parent, before } => {
                let parent_node = self.get_node_id(&parent).clone();
                let position = parent_node.children.iter().position(|&x| x == before);
                if !node_type {
                    self.document.attach_node_to_parent(node, parent, position);
                } else {
                    if position.is_some() {
                        // TODO add 1 or not ?
                        let last_node_id = parent_node.children[position.unwrap()];
                        if let NodeData::Text(TextData { ref mut value, .. }) = self
                            .document
                            .get_mut()
                            .get_node_by_id_mut(last_node_id)
                            .expect("node not found")
                            .data
                        {
                            value.push_str(&token.unwrap().to_string());
                            return;
                        };
                        self.document.attach_node_to_parent(node, parent, position);
                        return;
                    }
                    self.document.attach_node_to_parent(node, parent, position);
                }
            }
            InsertionPositionMode::LastChild(parent) => {
                if !node_type {
                    self.document.attach_node_to_parent(node, parent, None);
                } else {
                    let parent_node = self.get_node_id(&parent).clone();
                    if let Some(last_node_id) = parent_node.children.last() {
                        if let NodeData::Text(TextData { ref mut value, .. }) = self
                            .document
                            .get_mut()
                            .get_node_by_id_mut(*last_node_id)
                            .expect("node not found")
                            .data
                        {
                            value.push_str(&token.unwrap().to_string());
                            return;
                        };
                        self.document.attach_node_to_parent(node, parent, None);
                        return;
                    }
                    self.document.attach_node_to_parent(node, parent, None);
                }
            }
        }
    }

    pub fn insert_html_element(&mut self, token: &Token) -> NodeId {
        self.insert_node_element(token, None, Some(HTML_NAMESPACE))
    }

    pub fn insert_node_element(
        &mut self,
        token: &Token,
        override_node: Option<NodeId>,
        namespace: Option<&str>,
    ) -> NodeId {
        let mut node = self.create_node(token, namespace.unwrap_or(HTML_NAMESPACE));
        // add CSS classes from class attribute in element
        // e.g., <div class="one two three">
        // TODO: this will be refactored later in ElementAttributes to do this
        // when inserting a "class" attribute. Similar to "id" to attach it to the DOM
        // named_id_list. Although this will require some shared pointers

        if let NodeData::Element(ref mut element) = node.data {
            if element.attributes.contains("class") {
                if let Some(class_string) = element.attributes.get("class") {
                    element.classes = ElementClass::from_string(class_string);
                }
            }
        }

        let node_id = self.document.get_mut().add_new_node(node);
        let insert_position = self.appropriate_place_insert(override_node);
        self.insert_helper(node_id, insert_position, false, Some(token));

        //     if parser not created as part of html fragment parsing algorithm
        //       pop the top element queue from the relevant agent custom element reactions stack (???)

        // push element onto the stack of open elements so that is the new current node
        self.open_elements.push(node_id);

        // return element
        node_id
    }

    pub fn insert_document_element(&mut self, token: &Token) {
        let node = self.create_node(token, HTML_NAMESPACE);
        let node_id = self.document.get_mut().add_node(node, NodeId::root(), None);
        self.open_elements.push(node_id);
    }

    pub fn insert_comment_element(&mut self, token: &Token, insert_position: Option<NodeId>) {
        let node = self.create_node(token, HTML_NAMESPACE);
        if insert_position.is_some() {
            self.document
                .get_mut()
                .add_node(node, insert_position.unwrap(), None);
        } else {
            let node_id = self.document.get_mut().add_new_node(node);
            let insert_position = self.appropriate_place_insert(None);
            self.insert_helper(node_id, insert_position, false, Some(token));
        }
    }

    pub fn insert_text_element(&mut self, token: &Token) {
        let node = self.create_node(token, HTML_NAMESPACE);
        let node_id = self.document.get_mut().add_new_node(node);
        let insertion_position = self.appropriate_place_insert(None);
        // TODO, for text element, if the insertion_position is Docuement, should not do next step.
        self.insert_helper(node_id, insertion_position, true, Some(token));
    }

    pub fn appropriate_place_insert(
        &self,
        override_node: Option<NodeId>,
    ) -> InsertionPositionMode<NodeId> {
        let current_node_id = self.current_node_id();
        let target_id = override_node.unwrap_or(*current_node_id);
        let target_node = self.get_node_id(&target_id).clone();
        if !(self.foster_parenting
            && ["table", "tbody", "thead", "tfoot", "tr"].contains(&target_node.name.as_str()))
        {
            if target_node.name == "template" {
                panic!("current not support");
            } else {
                return InsertionPositionMode::LastChild(target_id);
            }
        }
        let mut iter = self.open_elements.iter().rev().peekable();
        while let Some(node_id) = iter.next() {
            let node = self.get_node_id(node_id);
            if node.name == "template" {
                panic!("current not support");
            } else if node.name == "table" {
                if node.parent.is_some() {
                    return InsertionPositionMode::Sibling {
                        parent: node.parent.unwrap(),
                        before: node_id.clone(),
                    };
                }
                // TODO has some question? can reached?
                return InsertionPositionMode::LastChild((*iter.peek().unwrap()).clone());
            }
        }
        return InsertionPositionMode::LastChild(*self.open_elements.first().unwrap());
    }

    pub fn adoption_agency_algorithm(&mut self, token: &Token) {
        // step 1
        let subject = match token {
            Token::StartTagToken { name, .. } | Token::EndTagToken { name, .. } => name,
            _ => panic!("un reached"),
        };
        let current_node_id = self.current_node_id().clone();

        // step 2
        if self.current_node().name == *subject
            && self.position_in_active_format(&current_node_id).is_none()
        {
            self.open_elements.pop();
            return;
        }

        // step 3
        let mut outer_loop_counter = 0;

        // step 4
        loop {
            // step 4.1
            if outer_loop_counter >= ADOPTION_AGENCY_OUTER_LOOP_DEPTH {
                return;
            }

            // step 4.2
            outer_loop_counter += 1;

            // step 4.3
            let (format_elem_idx, format_elem_node_id) = match self.find_format_element(subject) {
                None => {
                    return self.handle_in_body_any_other_end_tag();
                }
                Some((idx, node_id)) => (idx, node_id.clone()),
            };
            let format_elem_node = self.get_node_id(&format_elem_node_id);
            let format_ele_stack_position = match self
                .open_elements
                .iter()
                .rposition(|&x| x == format_elem_node_id)
            {
                // step 4.4
                None => {
                    self.parse_error("error");
                    self.active_formatting_elements.remove(format_elem_idx);
                    return;
                }
                Some(idx) => idx,
            };

            // step 4.5
            if !self.is_in_scope(&format_elem_node.name, Scope::Regular) {
                self.parse_error("error");
                return;
            }

            // step 4.6
            if format_elem_node_id != current_node_id {
                self.parse_error("error");
            }

            // step 4.7
            let (further_block_idx, further_block_node_id) =
                match self.find_further_block(format_ele_stack_position) {
                    // step 4.8
                    None => {
                        self.open_elements.truncate(format_ele_stack_position);
                        self.active_formatting_elements.remove(format_elem_idx);
                        return;
                    }
                    Some((idx, node_id)) => (idx, node_id.clone()),
                };

            // step 4.9
            let common_ancestor = self.open_elements[format_ele_stack_position - 1];

            // step 4.10
            let mut bookmark = format_elem_idx;

            // step 4.11
            let mut node_id = further_block_node_id;
            let mut last_node_id = further_block_node_id;
            let mut node_idx = further_block_idx;

            // step 4.12
            let mut inner_loop_counter = 0;

            // step 4.13
            loop {
                // step 4.13.1
                inner_loop_counter += 1;

                // step 4.13.2
                node_idx -= 1;
                node_id = self.open_elements[node_idx];

                // step 4.13.3
                if node_id == format_elem_node_id {
                    break;
                }

                // step 4.13.4
                if inner_loop_counter > ADOPTION_AGENCY_INNER_LOOP_DEPTH {
                    self.position_in_active_format(&node_id)
                        .map(|position| self.active_formatting_elements.remove(position));
                    self.open_elements.remove(node_idx);
                    continue;
                }
                // step 4.13.5
                let node_active_position = match self.position_in_active_format(&node_id) {
                    Some(pos) => pos,
                    None => {
                        self.open_elements.remove(node_idx);
                        continue;
                    }
                };

                // step 4.13.6
                let element = self.get_node_id(&node_id);
                let node_attributes = match element.data {
                    NodeData::Element(element) => element.attributes.clone_map(),
                    _ => HashMap::new(),
                };
                let replacement_node = Node::new_element(
                    &self.document,
                    &element.name,
                    node_attributes,
                    HTML_NAMESPACE,
                );
                let replace_node_id = self.document.get_mut().add_new_node(replacement_node);
                self.document.get_mut().attach_node_to_parent(
                    replace_node_id,
                    common_ancestor,
                    None,
                );

                self.active_formatting_elements[node_active_position] =
                    ActiveElement::Node(replace_node_id);

                self.open_elements[node_idx] = replace_node_id;

                node_id = replace_node_id;

                // step 4.13.7
                if last_node_id == further_block_node_id {
                    bookmark = node_active_position + 1;
                }

                // step 4.13.8
                self.document.relocate(last_node_id, node_id);

                // step 4.13.9
                node_id = last_node_id;
            }

            // step 4.14
            self.document.detach_node_from_parent(last_node_id);
            let insert_position = self.appropriate_place_insert(Some(common_ancestor));
            self.insert_helper(last_node_id, insert_position, false, None);

            // step 4.15
            let new_format_node: Node = Node::new_element(
                &self.document,
                &format_elem_node.name,
                HashMap::new(),
                HTML_NAMESPACE,
            );

            // step 4.16
            let node_id = self
                .document
                .get_mut()
                .add_new_node(new_format_node.clone());
            let further_block_node = self
                .document
                .get()
                .get_node_by_id(further_block_node_id)
                .expect("node not found")
                .clone();
            for child in further_block_node.children.iter() {
                self.document.get_mut().relocate(*child, node_id);
            }

            // step 4.17
            self.document
                .get_mut()
                .attach_node_to_parent(node_id, further_block_node_id, None);

            // step 4.18
            self.active_formatting_elements
                .insert(bookmark, ActiveElement::Node(node_id));
            let position = self.position_in_active_format(&format_elem_node_id);
            self.active_formatting_elements.remove(position.unwrap());

            // step 4.19
            self.open_elements.retain(|x| x == &format_elem_node_id);
            let position = self
                .position_in_open_element(&further_block_node_id);
            if position.is_some() {
            self.open_elements.insert(position.unwrap(), node_id);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::html5::input_stream::InputStream;
}

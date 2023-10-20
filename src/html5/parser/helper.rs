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
        self.open_elements
            .last()
            .expect("current_node_id not found")
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

    fn insert_element(&mut self, node: NodeId, override_node: Option<NodeId>) {
        let insertion_postion = self.appropriate_place_insert(override_node);
        match insertion_postion {
            InsertionPositionMode::Sibling { parent, before } => {
                self.document.detach_node_from_parent(node);
                let parent_node = self.get_node_id(&parent).clone();
                let position = parent_node
                    .children
                    .iter()
                    .position(|&x| x == before)
                    .unwrap();
                self.document
                    .attach_node_to_parent(node, parent, Some(position - 1));
            }
            InsertionPositionMode::LastChild(parent) => {
                self.document.detach_node_from_parent(node);
                self.document.attach_node_to_parent(node, parent, None);
            }
        }
    }

    fn swap_parent(&mut self, node: Node, parent: NodeId) -> NodeId {
        let node_id = self.document.get_mut().add_node(node, parent, None);
        let parent_node = self
            .document
            .get()
            .get_node_by_id(parent)
            .expect("node not found")
            .clone();
        for child in parent_node.children.iter() {
            if child == &node_id {
                continue;
            }
            self.document.get_mut().relocate(*child, node_id);
        }
        return node_id;
    }

    fn appropriate_place_insert(
        &self,
        override_node: Option<NodeId>,
    ) -> InsertionPositionMode<NodeId> {
        let current_node_id = self.current_node_id();
        let target_id = override_node.unwrap_or(*current_node_id);
        let target_node = self.get_node_id(&target_id).clone();
        if !(self.foster_parenting && [].contains(&target_node.name.as_str())) {
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
            }
        }
        return InsertionPositionMode::LastChild(*self.open_elements.first().unwrap());
    }

    fn adoption_agency_algorithm(&mut self, token: &Token) {
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
            let mut bookmark = format_elem_node_id.clone();

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
                let replace_node_id =
                    self.document
                        .get_mut()
                        .add_node(replacement_node, common_ancestor, None);

                self.active_formatting_elements[node_active_position] =
                    ActiveElement::Node(replace_node_id);

                self.open_elements[node_idx] = replace_node_id;

                node_id = replace_node_id;

                // step 4.13.7
                if last_node_id == further_block_node_id {
                    bookmark = match self.active_formatting_elements[node_active_position + 1] {
                        ActiveElement::Marker => panic!("not reached"),
                        ActiveElement::Node(node_id) => node_id.clone(),
                    }
                }

                // step 4.13.8
                self.document.relocate(node_id, last_node_id);

                // step 4.13.9
                node_id = last_node_id;
            }

            // step 4.14
            self.insert_element(last_node_id, Some(common_ancestor));

            // step 4.15
            let new_format_node: Node = Node::new_element(
                &self.document,
                &format_elem_node.name,
                HashMap::new(),
                HTML_NAMESPACE,
            );

            // step 4.16, 17
            let new_format_node_id = self.swap_parent(new_format_node, further_block_node_id);

            // step 4.18
            let position = self.position_in_active_format(&new_format_node_id).unwrap();
            self.active_formatting_elements.remove(position);
            let position = self.position_in_active_format(&bookmark).unwrap();
            self.active_formatting_elements
                .insert(position, ActiveElement::Node(new_format_node_id));

            // step 4.19
            let position = self.position_in_open_element(&format_elem_node_id).unwrap();
            self.open_elements.remove(position);
            let position = self
                .position_in_open_element(&further_block_node_id)
                .unwrap();
            self.open_elements.insert(position + 1, new_format_node_id);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::html5::input_stream::InputStream;
}

use crate::{NodeId, ui};

///
pub struct Controller<'a>(pub(crate) &'a mut ui::state::SnarlState);

impl Controller<'_> {
    /// Select one node.
    /// If `reset` is true, the previous selection will be cleared.
    pub fn select_one_node(&mut self, reset: bool, node: NodeId) {
        self.0.select_one_node(reset, node);
    }

    /// Deselect one node.
    pub fn deselect_one_node(&mut self, node: NodeId) {
        self.0.deselect_one_node(node);
    }

    /// Toggle selection of one node.
    pub fn toggle_node_selected(&mut self, node: NodeId) {
        if self.is_node_selected(node) {
            self.deselect_one_node(node);
        } else {
            self.select_one_node(false, node);
        }
    }

    /// Returns true if the given node is selected.
    pub fn is_node_selected(&self, node: NodeId) -> bool {
        self.0.selected_nodes().contains(&node)
    }

    /// Deselect all nodes.
    pub fn deselect_all_nodes(&mut self) {
        self.0.deselect_all_nodes();
    }

    /// Returns the selected nodes.
    pub fn selected_nodes(&self) -> &[NodeId] {
        self.0.selected_nodes()
    }
}

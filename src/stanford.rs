pub mod utils {
    
    use std::usize;
    
    /// Gets the indices of the next node label in a stanford formatted string.
    ///
    /// Assumes that the node is properly closed, i.e. ends in node separator or white space.
    /// Otherwise will panic.
    ///
    /// # Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `index_start_search` - The start index from where to search the tree for the next node label.
    /// * `separators` - Node separator characters.  
    pub fn get_next_node_label_indices(
        tree_in: &str,
        index_start_search: usize,
        separators: &[char],
    ) -> Result<(usize, usize), &'static str> {
        let index_first_alphabetic = tree_in[index_start_search..]
            .chars()
            .position(|c| !separators.contains(&c))
            .unwrap();
        let index_next_separator = tree_in[index_start_search + index_first_alphabetic..]
            .chars()
            .position(|c| separators.contains(&c))
            .unwrap();
        return Ok((
            index_first_alphabetic + index_start_search,
            index_next_separator + index_start_search,
        ));
    }

    
}

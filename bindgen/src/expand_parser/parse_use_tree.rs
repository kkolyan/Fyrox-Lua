use super::model::{Alias, Parser};

impl Parser {

    pub(crate) fn parse_use_tree(&mut self, tree: &syn::UseTree, use_path: Option<String>) {
        match tree {
            syn::UseTree::Path(it) => {
                self.parse_use_tree(&it.tree, Some(format!("{}::{}", use_path.unwrap().as_str(), it.ident.to_string())));
            },
            syn::UseTree::Name(it) => {
                self.global_aliases.insert(
					format!("{}::{}", &self.mod_stack.last().unwrap().full_path, it.ident.to_string()),
                    Alias::Use(format!("{}::{}", use_path.unwrap().as_str(), it.ident.to_string()))
                );
            }
            syn::UseTree::Rename(it) => {

                self.global_aliases.insert(
					format!("{}::{}", &self.mod_stack.last().unwrap().full_path, it.rename.to_string()),
                    Alias::Use(format!("{}::{}", use_path.unwrap().as_str(), it.ident.to_string()))
                );
            },
            syn::UseTree::Glob(_) => {
                self.global_wildcards.push((
					self.mod_stack.last().unwrap().full_path.clone(),
                    use_path.unwrap().clone(),
				));
            },
            syn::UseTree::Group(it) => {
                for sub_tree in it.items.iter() {
                    self.parse_use_tree(sub_tree, use_path.clone())
                }
            },
        }
    }
}

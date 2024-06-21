use crate::expand_parser::model::ModCtx;

use super::model::Parser;

impl Parser {
	pub fn parse_crate(&mut self, file: &syn::File, crate_name: &str) {
        self
            .mod_stack
            .push(ModCtx::new(crate_name.to_owned()));

        for item in file.items.iter() {
            self.parse_item(item);
        }
        self.mod_stack.pop().unwrap();
        assert_eq!(self.mod_stack.len(), 0);
	}
}
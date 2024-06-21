use syn::Visibility;

use super::model::{Alias, Function, GlobalName, Identifier, ModCtx, Parser};

impl Parser {
    pub(crate) fn parse_item(&mut self, item: &syn::Item) {
        match item {
            syn::Item::Const(_) => {}
            syn::Item::Enum(_) => {}
            syn::Item::ExternCrate(_) => {}
            syn::Item::Fn(it) => {
				if let Visibility::Public(_) = it.vis {
					let name = format!("{}::{}", &self.mod_stack.last().unwrap().full_path, it.sig.ident);
					self.result.functions.push(Function {
						name: name.clone(),
				        def: format!("{:?}", &it.sig),
					});
					self.known_objects.push(GlobalName(name))
				}
			}
            syn::Item::ForeignMod(_) => {}
            syn::Item::Impl(it) => {
			},
            syn::Item::Macro(_) => {}
            syn::Item::Mod(it) => {
                for item in it
                    .content
                    .as_ref()
                    .expect("expanded source is single file per crate")
                    .1
                    .iter()
                {
					self.push_mod(it.ident.to_string());
                    self.parse_item(item);
					self.pop_mod();
                }
            }
            syn::Item::Static(_) => {},
            syn::Item::Struct(_) => todo!(),
            syn::Item::Trait(_) => todo!(),
            syn::Item::TraitAlias(_) => todo!("trait aliases are not implemented"),
            syn::Item::Type(it) => {
				let name = format!("{}::{}", &self.mod_stack.last().unwrap().full_path, &it.ident);
                self.global_aliases.insert(
                    name.clone(),
                    Alias::Type(Identifier {
                        ident: it.ident.to_string(),
                        context_mod: self.mod_stack.last().unwrap().full_path.clone(),
                    }),
                );
				self.known_objects.push(GlobalName(name))
            }
            syn::Item::Union(_) => todo!("unions are not implemented"),
            syn::Item::Use(it) => {
                self.parse_use_tree(&it.tree, None);
            }
            syn::Item::Verbatim(_) => {}
            item => todo!("unexpected item: {:?}", item),
        }
    }
}

/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macros_helpers::get_crate_name;
use quote::quote;

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_db_fields_for_links_and_loaders(&mut self) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let db_field_name = field_receiver.db_field_name(&table_derive_attrs.casing()?)?;

        let serialized_field_fmt = quote!(#crate_name::Field::new(#db_field_name));

        self.serialized_fmt_db_field_names_instance
            .push(serialized_field_fmt.into());

        if !field_receiver.skip_serializing && !field_receiver.skip {
            match field_receiver.to_relation_type() {
                RelationType::LinkOne(_) => {
                    self.link_one_fields.push(serialized_field_fmt.into());
                    self.link_one_and_self_fields
                        .push(serialized_field_fmt.into());
                    self.linked_fields.push(serialized_field_fmt.into());
                }
                RelationType::LinkSelf(_) => {
                    self.link_self_fields.push(serialized_field_fmt.into());
                    self.link_one_and_self_fields
                        .push(serialized_field_fmt.into());
                    self.linked_fields.push(serialized_field_fmt.into());
                }
                RelationType::LinkMany(_) => {
                    self.link_many_fields.push(serialized_field_fmt.into());
                    self.linked_fields.push(serialized_field_fmt.into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}

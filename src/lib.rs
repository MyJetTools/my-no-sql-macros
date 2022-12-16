use proc_macro::TokenStream;

extern crate proc_macro;
mod my_no_sql_entity;

#[proc_macro_derive(
    BulkSelectDbEntity,
    attributes(db_field_name, json, bigint, line_no, sql_type,)
)]
pub fn postgres_bulk_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::my_no_sql_entity::generate(&ast)
}

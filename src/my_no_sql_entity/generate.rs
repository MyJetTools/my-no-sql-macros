use proc_macro::TokenStream;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_no_sql_server_abstractions::MyNoSqlEntity for ");
    result.push_str(struct_name.as_str());

    result.push_str(
        r#"{
        fn get_partition_key(&self) -> &str {
            &self.partition_key
        }
    
        fn get_row_key(&self) -> &str {
            &self.row_key
        }
    
        fn get_time_stamp(&self) -> i64 {
            DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
                .unwrap()
                .unix_microseconds
        }
    }"#,
    );

    result.parse().unwrap()
}

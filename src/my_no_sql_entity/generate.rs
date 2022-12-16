use proc_macro::TokenStream;

pub fn generate(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut result = input.to_string();
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_name = ast.ident.to_string();

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

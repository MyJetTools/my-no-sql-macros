use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;

pub fn generate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut src = input.to_string();
    let pos = find_struct_open(src.as_bytes());

    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let ident = ast.ident;


    quote!{
        #src

        impl my_no_sql_server_abstractions::MyNoSqlEntity for #ident {
            fn get_partition_key(&self) -> &str {
                &self.partition_key
            }
    
            fn get_row_key(&self) -> &str {
                &self.row_key
            }
    
            fn get_time_stamp(&self) -> i64 {
                rust_extensions::date_time::DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
                    .unwrap()
                    .unix_microseconds
            }
        }
    }.into()
}

/*

pub fn generate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut result = input.to_string();
    let pos = find_struct_open(result.as_bytes());

    if pos.is_none() {
        panic!("Open bracket of the structure is not found");
    }

    let mut params = get_params(attr.to_string());

    let table_name = params.remove("table_name");

    if table_name.is_none() {
        panic!("Please specify table_name parameter");
    }

    let table_name = table_name.unwrap();

    result.insert_str(
        pos.unwrap() + 1,
        r#"
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,"#,
    );

    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_name = ast.ident.to_string();

    result.push_str("impl my_no_sql_server_abstractions::MyNoSqlEntity for ");
    result.push_str(struct_name.as_str());

    result.push_str("{     const TABLE_NAME: &'static str = ");
    result.push_str(table_name.as_str());

    result.push_str(
        r#";
        fn get_partition_key(&self) -> &str {
            &self.partition_key
        }

        fn get_row_key(&self) -> &str {
            &self.row_key
        }

        fn get_time_stamp(&self) -> i64 {
            rust_extensions::date_time::DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
                .unwrap()
                .unix_microseconds
        }
    }"#,
    );

    result.parse().unwrap()
}
 */
fn find_struct_open(src: &[u8]) -> Option<usize> {
    for i in 0..src.len() {
        if src[i] == b'{' {
            return Some(i);
        }
    }

    None
}



fn get_params(attr: String) -> HashMap<String, String> {
    let count_commas = attr.chars().filter(|&c| c == ',').count();
    let count_dots = attr.chars().filter(|&c| c == '.').count();

    let separator = if count_commas > count_dots { ',' } else { '.' };

    let mut result = HashMap::new();
    for param in attr.split(separator) {
        let mut param = param.split('=');
        let key = param.next().unwrap().trim().to_string();
        let value = param.next().unwrap().trim().to_string();

        result.insert(key, value);
    }

    result
}


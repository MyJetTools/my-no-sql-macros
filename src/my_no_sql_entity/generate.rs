use proc_macro2::{Ident};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

use proc_macro::TokenStream;

pub fn generate(attr: TokenStream, input: TokenStream) -> TokenStream {

    let ast = proc_macro2::TokenStream::from(input);

    let mut result: Vec<proc_macro2::TokenTree> = Vec::new();

    let mut struct_name = None;
    let mut passed_struct_name = false;

    let mut params = get_params(attr.to_string());

    let table_name = params.remove("table_name");

    if table_name.is_none() {
        panic!("Please specify table_name parameter");
    }

    let table_name = table_name.unwrap();

    for item in ast{

        if struct_name.is_none(){
            if let proc_macro2::TokenTree::Ident(ident) = &item{
                    if passed_struct_name{
                            struct_name = Some(ident.clone());
                    }
                    else{
                        if ident.to_string() == "struct"{
                            passed_struct_name = true;
                        }
                    }
                }
                result.push(item);
            }
            else{
 
                    if let proc_macro2::TokenTree::Group(group) = &item{
                        if group.delimiter() == proc_macro2::Delimiter::Brace{
                            let mut first = true;
    
                            let mut result_tokens: Vec<proc_macro2::TokenTree> = Vec::new();

                            for token in group.stream(){
                                if first{
                                    let token:proc_macro2::TokenStream = 
                                    quote!{
                                        #[serde(rename = "PartitionKey")]
                                        pub partition_key: String,
                                    }.into();
                                    result_tokens.extend(token);

                                    let token:proc_macro2::TokenStream = 
                                    quote!{
                                        #[serde(rename = "RowKey")]
                                        pub row_key: String,
                                    }.into();
                                    result_tokens.extend(token);


                                    let token:proc_macro2::TokenStream = 
                                    quote!{
                                        #[serde(rename = "TimeStamp")]
                                        pub time_stamp: String,
                                    }.into();
                                    result_tokens.extend(token);
    
    
                                    first = false;
                                }
                                result_tokens.push(token);
                            }
    
                            result.push(proc_macro2::TokenTree::Group(proc_macro2::Group::new(proc_macro2::Delimiter::Brace, result_tokens.into_iter().collect())));
                        }
                    }

                }
        

    }

    let struct_name = struct_name.unwrap();

    quote!{
     
        #(#result)*

        impl my_no_sql_server_abstractions::MyNoSqlEntity for #struct_name {

            const TABLE_NAME: &'static str = #table_name;

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





fn get_params(attr: String) -> HashMap<String, String> {
    let count_commas = attr.chars().filter(|&c| c == ',').count();
    let count_semi = attr.chars().filter(|&c| c == ';').count();

    let separator = if count_commas > count_semi { ',' } else { '.' };

    let mut result = HashMap::new();
    for param in attr.split(separator) {
        let mut param = param.split('=');
        let key = param.next().unwrap().trim().to_string();
        let mut value = param.next().unwrap().trim();

        if value.starts_with('"'){
            value = &value[1..value.len()-1];
        }

        if value.starts_with('\''){
            value = &value[1..value.len()-1];
        }

        result.insert(key, value.to_string());
    }

    result
}


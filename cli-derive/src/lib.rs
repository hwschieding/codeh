use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(ArgParsing, attributes(opt, flag))]
pub fn arg_parsing_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("ArgPassing should be placed on structs"),
    };

    let mut is_opt_arms = Vec::new();
    let mut set_opt_arms = Vec::new();
    let mut is_flag_arms = Vec::new();
    let mut set_flag_arms = Vec::new();

    for f in fields {
        let field_name = match f.ident.as_ref() {
            Some(n) => n,
            None => continue,
        };
        let name = field_name.to_string();

        if f.attrs.iter().any(|a| a.path().is_ident("opt")) {
            is_opt_arms.push(quote! {
                #name => { true },
            });
            set_opt_arms.push(quote! {
                #name => { self.#field_name = Some(value); true },
            });
        } else if f.attrs.iter().any(|a| a.path().is_ident("flag")) {
            is_flag_arms.push(quote! {
                #name => { true },
            });
            set_flag_arms.push(quote! {
                #name => { self.#field_name = true; true },
            });
        }
    }

    let expanded = quote! {
        impl #struct_name
        {
            pub fn set_opt(&mut self, opt_name: &str, value: String) -> bool
            {
                match opt_name {
                    #(#set_opt_arms)*
                    _ => false
                }
            }
            pub fn set_flag(&mut self, flag_name: &str) -> bool
            {
                match flag_name
                {
                    #(#set_flag_arms)*
                    _ => false
                }
            }
            pub fn is_opt(opt_name: &str) -> bool
            {
                match opt_name
                {
                    #(#is_opt_arms)*
                    _ => false
                }
            }
            pub fn is_flag(flag_name: &str) -> bool
            {
                match flag_name
                {
                    #(#is_flag_arms)*
                    _ => false
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(RunCommand)]
pub fn run_command_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("RunCommand should be placed on enum"),
    };

    let variant_arms = variants.iter().map(|v| {
        if !matches!(v.fields, syn::Fields::Unit) {
            panic!("RunCommand can only be used with unit variants");
        }

        let var_name = &v.ident;
        let name = var_name.to_string().to_lowercase();

        quote! {
            #name => Some(Self::#var_name),
        }
    });

    let expanded = quote! {
        impl #enum_name {
            pub fn from_name(name: &str) -> Option<Self>{
                match name {
                    #(#variant_arms)*
                    _ => None
                }
            }
        }
    };

    TokenStream::from(expanded)
}

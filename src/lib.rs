use core::sync;
use std::collections::HashMap;
use std::sync;

use proc_macro::{TokenStream};
use quote::{ToTokens, format_ident};
use syn::{Data, DataStruct, DeriveInput, Expr, Ident, ItemFn, Stmt, Type, parse};
use syn::{parse_macro_input, LitStr};

fn impl_meta(ast: DeriveInput) -> TokenStream{
    let ast_name = &ast.ident;
    let name = ast_name.to_string();
    let mut dtypes: Vec<String> = vec![];
    let mut members = vec![];
    match ast.data{
        Data::Struct(m) => {
            m.fields.iter().for_each(|x| {
                let nm = x.ident.as_ref().map(|x| x.to_string()).unwrap_or_default();
                members.push(nm);
                dtypes.push(x.ty.to_token_stream().to_string());
            });
        }
        Data::Enum(m) => {
            m.variants.iter().for_each(|x|{
                
            });
        }
        Data::Union(m) => {}
    };

let entries = dtypes.iter().zip(members.iter()).map(|(dtype, attrib)| {
    quote::quote! {
        (#dtype, #attrib)
    }
});

    quote::quote! {
        impl StructMetaData for #ast_name{
            fn name(&self) -> &'static str{
                #name
            }

            fn members(&self) -> &[&'static str]{
                &[#(#members),*]
            }

            fn member_types(&self) -> &[&'static str]{
                &[#(#dtypes),*]
            }

            fn member_map(&self) -> &[(&'static str,&;static str)]{
                &[#(#entries),*]
            }
        }
        
    }.into()
}



#[proc_macro_derive(StructMetaData)]
pub fn struct_meta(strm: TokenStream) -> TokenStream{
    let ast = syn::parse::<DeriveInput>(strm).unwrap();
    impl_meta(ast)
}



#[proc_macro]
pub fn TYPE(input: TokenStream) -> TokenStream {
    let ty: Type = match syn::parse_str(&parse_macro_input!(input as LitStr).value()) {
        Ok(ty) => ty,
        Err(e) => return e.to_compile_error().into(),
    };

    quote::quote! {
        #ty
    }.into()
}

macro_rules! time {
    ($x:expr) => { 
        let st = time.now();
        $x
        time.now() - st
    };
}

struct Attrib_macro_param{
    attribs: HashMap<String,Expr>
}

impl syn::parse::Parse for Attrib_macro_param{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ret = HashMap::new();
        while !input.is_empty(){
            let k: Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let v: Expr = input.parse()?;
            ret.insert(k.to_string(), v);
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Attrib_macro_param { attribs: ret })
    }
}


#[proc_macro_attribute]
pub fn generate_getter_setters(_attr: TokenStream,body: TokenStream) -> TokenStream{
    let dip = parse_macro_input!(body as DeriveInput);
    handle_dip_get_set(dip)
}

fn handle_dip_get_set(dip: DeriveInput) -> TokenStream{
    let ident = dip.ident;
    let ident_s = ident.to_string();    
    let mut dtypes: Vec<String> = vec![];
    let mut members = vec![];
    match dip.data{
        Data::Struct(m) => {
            m.fields.iter().for_each(|x| {
                let nm = x.ident.as_ref().map(|x| x.to_string()).unwrap_or_default();
                members.push(nm);
                dtypes.push(x.ty.to_token_stream().to_string());
            });
        }
        Data::Enum(m) => {panic!("Enums not supported for getter setter generation")}
        Data::Union(m) => {panic!("Enums not supported for getter setter generation")}
    };

    let functions = members.iter().zip(dtypes.iter()).map(|(member, ret)| {
            let getter = format_ident!("get_{}", member);
            let setter = format_ident!("set_{}", member);

            quote::quote! {
                fn #getter(&self) -> #ret {
                    self.#member
                }

                fn #setter(&mut self,v: #ret){
                    self.#member = #ret
                }
            }
        });

    quote::quote! {
        impl #ident_s {
            #(#functions)*
        }
    }.into()
}



#[proc_macro_attribute]
fn validate_body(attrs: TokenStream,body: TokenStream) -> TokenStream{
    let args = parse_macro_input!(attrs as Attrib_macro_param);
    let body = parse_macro_input!(body as ItemFn);
    validate_body(body,args);
}


fn validate_body(strm: ItemFn,limits: Attrib_macro_param) -> TokenStream{
    strm.block.stmts.iter().for_each(|x|{
        match x{
            Stmt::Local(x) =>{
                
                if let Some(init) = x.init{
                    init.expr
                }
            }
        }
    });


}




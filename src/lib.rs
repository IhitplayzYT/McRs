use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, DataStruct, DeriveInput, Type};


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


use syn::{parse_macro_input, LitStr};

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

#[proc_macro]
pub fn sz_type(strm: TokenStream) -> TokenStream{
    let input = parse_macro_input!(strm with syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated);
    let var = &input[0];
    let sz = &input[1];
    


}





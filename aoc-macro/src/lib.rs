#![recursion_limit = "128"]
#![feature(try_blocks, proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{Expr, Token, Ident, Path};
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ItemFn};
use syn::{parse_macro_input};

#[derive(Debug)]
struct TestParams {
    name: Ident,
    expr: Expr,
    expect: Expr,
}

impl Parse for TestParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let expect = input.parse()?;
        Ok(TestParams {
            name,
            expr,
            expect,
        })
    }
}

#[proc_macro]
pub fn sol_test(input: TokenStream) -> TokenStream {
    let TestParams {
        name, expr, expect,
    } = parse_macro_input!(input as TestParams);
    (quote! {
        #[test]
        fn #name() {
            assert_eq!(#expr, #expect);
        }
    }).into()
}

#[proc_macro_attribute]
pub fn generator(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let bench_generator_ident = Ident::new("bench_gen", input.span());

    let r = quote! {
        pub fn #bench_generator_ident(name: &str, c: &mut criterion::Criterion) {
            c.bench_function(
                name,
                |b| {
                    b.iter(|| generator(INPUT));
                }
            );
        }
        #input
    };
    r.into()
}

mod kw {
    use syn::custom_keyword;
    custom_keyword!(expect);
    custom_keyword!(example);
    custom_keyword!(example_input);
}

#[derive(Debug)]
struct SolutionParams {
    part: Ident,
    example_input: Expr,
    example: Expr,
    expected: Option<Expr>,
}

impl Parse for SolutionParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let part: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<kw::example_input>()?;
        input.parse::<Token![=]>()?;
        let example_input = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<kw::example>()?;
        input.parse::<Token![=]>()?;
        let example = input.parse()?;
        let expected = if input.peek2(kw::expect) {
            input.parse::<Token![,]>()?;
            input.parse::<kw::expect>()?;
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(SolutionParams {
            part,
            example_input,
            example,
            expected,
        })
    }
}

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let fn_name = input.ident.clone();
    let ret = input.decl.output.clone();
    let args = input.decl.inputs.len();

    let example_arg = if args == 2 {
        quote!(true)
    } else {
        quote!()
    };

    let normal_arg = if args == 2 {
        quote!(false)
    } else {
        quote!()
    };

    let SolutionParams {
        part,
        example_input,
        example,
        expected,
    } = parse_macro_input!(attr as SolutionParams);
    if part == "part1" || part == "part2" {
        let example_ident = Ident::new(&format!("{}_example", part), part.span());
        let complete_ident = Ident::new(&format!("{}_complete", part), part.span());

        let example_func = quote! {
            fn #example_ident() {
                let out = #fn_name(#example_input, #example_arg);
                assert_eq!(out, #example);
            }
        };
        let r = match ret {
            syn::ReturnType::Default => quote!(-> Option<()>),
            syn::ReturnType::Type(arrow, ty) => quote!(#arrow Option<#ty>),
        };
        let complete_func = if expected.is_none() {
            quote! {
                pub fn #complete_ident() #r {
                    #example_ident();
                    let out = #fn_name(generator(INPUT), #normal_arg);
                    Some(out)
                }
            }
        } else {
            quote! {
                pub fn #complete_ident() #r {
                    #example_ident();
                    let out = #fn_name(generator(INPUT), #normal_arg);
                    assert_eq!(out, #expected);
                    Some(out)
                }
            }
        };

        let bench_soln_ident = Ident::new(&format!("{}_bench_soln", part), part.span());

        let res = quote! {
            pub fn #bench_soln_ident(name: &str, c: &mut criterion::Criterion) {
                c.bench_function(
                    name,
                    |b| {
                        b.iter_with_large_setup(
                            || generator(INPUT),
                            |data| {
                                #fn_name(data, #normal_arg);
                            });
                    }
                );
            }
            #example_func
            #complete_func
            #input
        };

        TokenStream::from(res)
    } else {
        part.span().unstable().error("Only `part1`/`part2` supported").emit();
        TokenStream::new()
    }
}

#[proc_macro]
pub fn days(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Path, Token![,]>::parse_terminated;
    let paths = parser.parse(input).unwrap();

    let modules = paths.clone().iter()
        .map(|path| {
            let module = path.segments.first().unwrap().value().ident.clone();
            quote_spanned!(module.span()=> mod #module;)
        }).collect::<TokenStream2>();

    let run_parts = paths.clone().iter()
        .map(|path| {
            let day_name = path.segments.first().unwrap().value().ident.clone();
            let day_n = day_name.to_string()[3..].parse::<u8>().unwrap();
            let body = match path.segments.len() {
                1 => quote! {
                    let args = std::env::args().collect::<Vec<_>>();
                    for window in args.windows(2) {
                        let arg1 = &window[0];
                        let arg2 = &window[1];
                        if arg1.contains(stringify!(#day_name)) {
                            if arg2.contains("part1") || arg2.contains("both") {
                                match #path::part1_complete() {
                                    Some(x) => {
                                        eprintln!("Day {} Part 1: {}", #day_n, x);
                                    }
                                    None => {
                                        eprintln!("Day {} Part 1: Not yet implemented", #day_n);
                                    }
                                }
                            }
                            if arg2.contains("part2") || arg2.contains("both") {
                                match #path::part2_complete() {
                                    Some(x) => {
                                        eprintln!("Day {} Part 2: {}", #day_n, x);
                                    }
                                    None => {
                                        eprintln!("Day {} Part 2: Not yet implemented", #day_n);
                                    }
                                }
                            }
                        }
                    }
                },
                _ => {
                    path.span().unstable().error("only one segments allowed").emit();
                    unimplemented!()
                }
            };
            quote!(fn #day_name() { #body })
        }).collect::<TokenStream2>();

    let call_days = paths.clone().iter().map(|path| {
        let ident = path.segments.first().unwrap().value().ident.clone();
        quote! {
            #ident();
        }
    }).collect::<TokenStream2>();

    let bench_days = paths.clone().iter().map(|path| {
        let ident = path.segments.first().unwrap().value().ident.clone();
        quote! {
            #ident::bench_gen(&format!("{}::generator", stringify!(#ident)), &mut c);
            #ident::part1_bench_soln(&format!("{}::part1", stringify!(#ident)), &mut c);
            #ident::part2_bench_soln(&format!("{}::part2", stringify!(#ident)), &mut c);
        }
    }).collect::<TokenStream2>();

    let res = quote! {
        #modules

        #run_parts

        fn main() {
            let arg = std::env::args().nth(1);
            match arg.as_ref().map(|s| s.as_str()) {
                Some("run") => {
                    #call_days
                }
                Some(_) | None => {
                    criterion::init_logging();
                    let mut c = criterion::Criterion::default()
                        .configure_from_args();
                    #bench_days
                }
            }
        }
    };

    TokenStream::from(res)
}

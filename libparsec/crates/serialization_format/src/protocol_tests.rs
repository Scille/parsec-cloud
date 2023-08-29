// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::*;

/// See [crate::generate_protocol_cmds_family_from_contents]
pub(crate) fn generate_protocol_cmds_tests(cmds: Vec<JsonCmd>, family_name: &str) -> TokenStream {
    let family = &GenCmdsFamily::new(cmds, family_name, ReuseSchemaStrategy::Never);
    let family_name = format_ident!("{}", &family.name);
    let versioned_cmds = family
        .versions
        .iter()
        .sorted_by_key(|(v, _)| *v)
        .map(|(version, cmds)| quote_versioned_cmds_test(*version, family, cmds));

    quote! {
        pub mod #family_name {
            #(#versioned_cmds)*
        }
    }
}

fn quote_versioned_cmds_test(version: u32, family: &GenCmdsFamily, cmds: &[GenCmd]) -> TokenStream {
    let mod_version = format_ident!("v{version}");
    let family_name = format_ident!("{}", &family.name);
    let cmd_tests = cmds.iter().map(|cmd| quote_cmd_tests(version, cmd));

    quote! {
        // The secret sauce is here! the cmd module will access the schema by
        // doing e.g. `use super::authenticated_cmds` (instead of the regular
        // `libparsec_protocol::authenticated_cmd::vX`) this allows to reuse the
        // cmd module multiple times (i.e. not `use` it but actually compiling it),
        // each time with a parent module defining a different version of the cmd family
        pub mod #mod_version {
            use libparsec_protocol::#family_name::#mod_version as #family_name;
            use libparsec_tests_fixtures::prelude::*;

            #(#cmd_tests)*
        }
    }
}

fn quote_cmd_tests(_cmd_version: u32, cmd: &GenCmd) -> TokenStream {
    let module_name = &cmd.cmd;
    let cmd_mod = format_ident!("{}", module_name);

    // Command schemas are not reused for tests generation
    match &cmd.spec {
        GenCmdSpec::Original { req, reps, .. } => {
            let req_tests = quote_cmd_req_tests(module_name, req);
            let rep_tests: Vec<TokenStream> = reps
                .iter()
                .map(|rep| quote_cmd_rep_tests(module_name, rep))
                .collect_vec();

            quote! {
                pub mod #cmd_mod;

                #req_tests

                #(#rep_tests)*
            }
        }
        // Command specs should not be reused for test generation
        _ => panic! {"Should not reuse command spec for: {}", module_name},
    }
}

fn quote_cmd_req_tests(module_name: &String, req: &GenCmdReq) -> TokenStream {
    // request test function name is prefixed with command name (e.g. "invite_new_req")
    let cmd_mod = format_ident!("{}", module_name);
    let test_function = format_ident!("{}_req", req.cmd);
    quote! {
        // No distinction between request kinds "Unit" or "Composed",
        // only one test function is generated for the request
        #[parsec_test]
        fn #test_function(){
            #cmd_mod::req()
        }
    }
}

fn quote_cmd_rep_tests(module_name: &String, rep: &GenCmdRep) -> TokenStream {
    // response function name contains the status (e.g "rep_ok")
    // response test function name is prefixed with module_name (e.g. "invite_new_rep_ok")
    let cmd_mod = format_ident!("{}", module_name);
    let rep_function = format_ident!("rep_{}", rep.status);
    let test_function = format_ident!("{}_{}", cmd_mod, rep_function);

    quote! {
        // No distinction between response kinds "Unit" or "Composed",
        // only one test function is generated for each response status
        #[parsec_test]
        fn #test_function(){
            #cmd_mod::#rep_function()
        }
    }
}

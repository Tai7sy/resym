mod frontend;
mod resymc_app;
mod resymc_options;
mod syntax_highlighting;

use anyhow::Result;
use resym_core::pdb_types::PrimitiveReconstructionFlavor;
use structopt::StructOpt;

use crate::resymc_app::ResymcApp;
use crate::resymc_options::ResymcOptions;

const DEFAULT_PRIMITIVE_FLAVOR: PrimitiveReconstructionFlavor = PrimitiveReconstructionFlavor::Msvc;

fn main() -> Result<()> {
    env_logger::init();
    let app = ResymcApp::new()?;

    // Process command and options
    let opt = ResymcOptions::from_args();
    match opt {
        ResymcOptions::List {
            pdb_path,
            type_name_filter,
            output_file_path,
            case_insensitive,
            use_regex,
            ignore_std_types,
        } => app.list_types_command(
            pdb_path,
            type_name_filter,
            case_insensitive,
            use_regex,
            ignore_std_types,
            output_file_path,
        ),
        ResymcOptions::Dump {
            pdb_path,
            type_name,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_dependencies,
            print_access_specifiers,
            integers_as_hexadecimal,
            ignore_std_types,
            highlight_syntax,
        } => app.dump_types_command(
            pdb_path,
            Some(type_name),
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_dependencies,
            print_access_specifiers,
            integers_as_hexadecimal,
            ignore_std_types,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::DumpAll {
            pdb_path,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_access_specifiers,
            integers_as_hexadecimal,
            ignore_std_types,
            highlight_syntax,
        } => app.dump_types_command(
            pdb_path,
            None,
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            false,
            print_access_specifiers,
            integers_as_hexadecimal,
            ignore_std_types,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::Diff {
            from_pdb_path,
            to_pdb_path,
            type_name,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_dependencies,
            print_access_specifiers,
            integers_as_hexadecimal,
            ignore_std_types,
            highlight_syntax,
        } => app.diff_type_command(
            from_pdb_path,
            to_pdb_path,
            type_name,
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_dependencies,
            print_access_specifiers,
            integers_as_hexadecimal,
            ignore_std_types,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::ListModules {
            pdb_path,
            module_path_filter,
            output_file_path,
            case_insensitive,
            use_regex,
        } => app.list_modules_command(
            pdb_path,
            module_path_filter,
            case_insensitive,
            use_regex,
            output_file_path,
        ),
        ResymcOptions::DumpModule {
            pdb_path,
            module_id,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_access_specifiers,
            highlight_syntax,
        } => app.dump_module_command(
            pdb_path,
            module_id,
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_access_specifiers,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::DiffModule {
            from_pdb_path,
            to_pdb_path,
            module_path,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_access_specifiers,
            highlight_syntax,
        } => app.diff_module_command(
            from_pdb_path,
            to_pdb_path,
            module_path,
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_access_specifiers,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::ListSymbols {
            pdb_path,
            symbol_name_filter,
            output_file_path,
            case_insensitive,
            use_regex,
            ignore_std_types,
        } => app.list_symbols_command(
            pdb_path,
            symbol_name_filter,
            case_insensitive,
            use_regex,
            ignore_std_types,
            output_file_path,
        ),
        ResymcOptions::DumpSymbol {
            pdb_path,
            symbol_name,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_access_specifiers,
            highlight_syntax,
        } => app.dump_symbol_command(
            pdb_path,
            Some(symbol_name),
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_access_specifiers,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::DumpAllSymbols {
            pdb_path,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_access_specifiers,
            highlight_syntax,
        } => app.dump_symbol_command(
            pdb_path,
            None,
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_access_specifiers,
            highlight_syntax,
            output_file_path,
        ),
        ResymcOptions::DiffSymbol {
            from_pdb_path,
            to_pdb_path,
            symbol_name,
            output_file_path,
            primitive_types_flavor,
            print_header,
            print_access_specifiers,
            highlight_syntax,
        } => app.diff_symbol_command(
            from_pdb_path,
            to_pdb_path,
            symbol_name,
            primitive_types_flavor.unwrap_or(DEFAULT_PRIMITIVE_FLAVOR),
            print_header,
            print_access_specifiers,
            highlight_syntax,
            output_file_path,
        ),
    }
}

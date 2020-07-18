use clap::{App, AppSettings, Arg, SubCommand};

use crate::types::{Fetcher, Template};

pub fn build_cli() -> App<'static, 'static> {
    App::new("nix-template")
        .version("0.1")
        .about("Create common nix expressions")
        .setting(AppSettings::ColoredHelp)
        // make completions and other subcommands distinct from
        // default template usage
        .setting(AppSettings::SubcommandsNegateReqs)
        // make it so that completions subcommand doesn't
        // inherit global options
        .setting(AppSettings::ArgsNegateSubcommands)
        .after_help(
            "EXAMPLES:

$ nix-template python --pname requests -f pypi pkgs/development/python-modules/

",
        )
        .arg(
            Arg::from_usage("<TEMPLATE> 'Language or framework template target'")
                .possible_values(&Template::variants())
                .case_insensitive(true)
                .default_value("stdenv"),
        )
        .arg(
            Arg::from_usage("[PATH] 'location for file to be written'")
                .default_value("default.nix")
                .default_value_if("TEMPLATE", Some("mkshell"), "shell.nix"),
        )
        .arg(Arg::from_usage(
            "-p,--pname [pname] 'Package name to be used in expresion'",
        ))
        .arg(Arg::from_usage(
            "-n,--nixpkgs 'Intended be used within nixpkgs, will append pname to file path, and print addition statement'",
        ).takes_value(false))
        .arg(
            Arg::from_usage("-f,--fetcher [fetcher] 'Fetcher to use'")
                .possible_values(&Fetcher::variants())
                .case_insensitive(true)
                .default_value("github")
                .default_value_if("TEMPLATE", Some("python"), "pypi"),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completion scripts, writes to stdout")
                .arg(
                    Arg::from_usage("<SHELL>")
                        .case_insensitive(true)
                        .possible_values(&clap::Shell::variants()),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python() {
        let m = build_cli().get_matches_from(vec!["nix-template", "python", "-n", "-p", "requests"]);
        println!("{:?}", m);
        assert_eq!(m.value_of("pname"), Some("requests"));
        assert_eq!(m.value_of("TEMPLATE"), Some("python"));
        assert_eq!(m.value_of("fetcher"), Some("pypi"));
        assert_eq!(m.occurrences_of("nixpkgs"), 1);
    }

    #[test]
    fn test_mkshell() {
        let m = build_cli().get_matches_from(vec!["nix-template", "mkshell"]);
        assert_eq!(m.value_of("TEMPLATE"), Some("mkshell"));
        assert_eq!(m.value_of("PATH"), Some("shell.nix"));
    }

    #[test]
    fn test_fetcher() {
        let m = build_cli().get_matches_from(vec!["nix-template", "-f", "gitlab"]);
        assert_eq!(m.value_of("fetcher"), Some("gitlab"));
    }
}
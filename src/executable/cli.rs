use clap::{Parser, Subcommand};

#[derive(Debug, PartialEq, Parser)]
pub struct CLIArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum Command {
    #[clap(name = "man")]
    Manager,
    #[clap(name = "mod", about = "Modify an existing package")]
    Modifier {
        #[clap(subcommand)]
        operation: ModiferOperation,
    },
    #[cfg(feature = "with-info")]
    #[clap(name = "info", about = "Report Information about a package's contents")]
    Info {
        #[clap(subcommand)]
        operation: InformationOperation,
    },
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum ModiferOperation {
    #[clap(
        short_flag = 'r',
        long_flag = "rm-chk",
        about = "Remove the checksum from an existing package"
    )]
    RemoveChecksum {
        #[clap(
            conflicts_with = "path",
            short,
            help = "The name of the package as stored in the management DB",
            required_unless_present = "path"
        )]
        name: Option<String>,
        #[clap(
            conflicts_with = "name",
            short,
            help = "The path to the archer zip file",
            required_unless_present = "name"
        )]
        path: Option<String>,
        #[clap(short, help = "The path to the output zip file")]
        output_path: Option<String>,
        #[clap(short, long, help = "Show verbose output")]
        verbose: bool,
    },
    #[clap(
        short_flag = 'a',
        long_flag = "add-chk",
        about = "Generate and add the checksum to an existing package"
    )]
    AddChecksum {
        #[clap(
            conflicts_with = "path",
            short,
            help = "The name of the package as stored in the management DB"
        )]
        name: Option<String>,
        #[clap(short, help = "Remove the checksum if present in the zip file")]
        remove_checksum: bool,
        #[clap(
            short,
            help = "The path to the archer zip file",
            required_unless_present = "name",
            conflicts_with = "name"
        )]
        path: Option<String>,
        #[clap(short, help = "The path to the output zip file")]
        output_path: Option<String>,
    },
    #[clap(
        short_flag = 'm',
        long_flag = "mk-pkg",
        about = "Create a new package from a directory"
    )]
    MakePackage {
        #[clap(
            short = 'a',
            long = "add",
            help = "Add the package to the database",
            requires = "name"
        )]
        add_to_db: bool,
        input_directory: String,
        #[clap(
            long = "name",
            help = "Specify the name of the package if adding to the db"
        )]
        name: Option<String>,
        #[clap(long = "version", help = "Specify the version of the package")]
        version: Option<String>,
        #[clap(
            name = "output_path",
            short = 'o',
            long = "output",
            help = "Specify the output path for the package"
        )]
        output_path: Option<String>,
        #[clap(short, long, help = "Show verbose output")]
        verbose: bool,
    },
}

#[cfg(feature = "with-info")]
#[derive(Debug, PartialEq, Subcommand)]
pub enum InformationOperation {
    #[clap(
        short_flag = 'o',
        name = "overview",
        about = "Print information about a package"
    )]
    Overview {
        #[clap(short, help = "Lists package contents in increased detail")]
        detailed: bool,
        #[clap(short, help = "Prints processing information")]
        verbose: bool,
        #[clap(
            short,
            long,
            help = "Specify the package file to process",
            required_unless_present = "name"
        )]
        path: Option<String>,
        #[clap(short, long, help = "Specify a package name stored in the database")]
        name: Option<String>,
    },
    #[clap(
        short_flag = 'a',
        name = "apps",
        about = "Print information about a package"
    )]
    Applications {
        #[clap(short = 'l', long = "list-apps", help = "Lists all applications")]
        list_applications: bool,
        #[clap(
            short = 'a',
            long = "list-aw",
            help = "Lists all applications with advanced workflow"
        )]
        list_aw: bool,
        #[clap(
            short,
            long,
            help = "Specify the package file to process",
            required_unless_present = "name"
        )]
        path: Option<String>,
        #[clap(short, long, help = "Specify a package name stored in the database")]
        name: Option<String>,
    },
    Datafeeds,
    Notifications,
    Solutions,
    Dashboards,
    Workspaces,
    DataDrivenEvents,
    Levels,
}

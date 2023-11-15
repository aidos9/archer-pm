use archer_package_manager::error::APMError;
use archer_package_manager::packages::processing::{LoadedPackage, Package};
use archer_package_manager::zip_manipulation::read_archive;

use crate::cli::{ExportDataFormat, InformationOperation};
use crate::tui::{print_section_title, print_separator_line};
use crate::util::package_path;

const DATAFEED_SECTION_TITLE: &'static str = "Datafeeds";
const MANIFEST_SECTION_TITLE: &'static str = "Package Details";

pub fn execute_informer_op(op: InformationOperation, name: Option<String>, path: Option<String>) {
    match execute_op(op, package_path(name, path)) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error - {}", e.error_type());
            eprintln!("Description: {}", e.description());
        }
    }
}

fn execute_op(operation: InformationOperation, path: String) -> Result<(), APMError> {
    match operation {
        InformationOperation::Overview { detailed } => {
            let pkg = Package::new(read_archive(&path)?)
                .with_data_feeds()
                .load()?;

            print_manifest_details(&pkg);
            print_datafeeds(&pkg, detailed);
        }
        InformationOperation::Applications {
            list_applications,
            list_aw,
        } => todo!("This mode is currently unsupported"),
        InformationOperation::Datafeeds {
            list_all,
            list_detailed,
            format,
        } => {
            if list_all {}

            match format {
                ExportDataFormat::Readable => todo!(),
                ExportDataFormat::JSON => todo!(),
                ExportDataFormat::XML => todo!(),
                ExportDataFormat::TOML => todo!(),
            }
        }
        InformationOperation::Notifications => todo!("This mode is currently unsupported"),
        InformationOperation::Solutions => todo!("This mode is currently unsupported"),
        InformationOperation::Dashboards => todo!("This mode is currently unsupported"),
        InformationOperation::Workspaces => todo!("This mode is currently unsupported"),
        InformationOperation::DataDrivenEvents => todo!("This mode is currently unsupported"),
        InformationOperation::Levels => todo!("This mode is currently unsupported"),
    }

    return Ok(());
}

fn print_manifest_details(pkg: &LoadedPackage) {
    print_section_title(MANIFEST_SECTION_TITLE);
    println!("Package Name: {}", pkg.manifest().package_name);
    println!("GUID: {}", pkg.manifest().package_guid);
    println!("Description: {}", pkg.manifest().package_description);
    println!("Version: {}", pkg.manifest().package_version);
    println!("Date: {}", pkg.manifest().package_date);
    println!("Provider: {}", pkg.manifest().package_provider);
}

fn print_datafeeds(pkg: &LoadedPackage, detailed: bool) {
    print_section_title(DATAFEED_SECTION_TITLE);

    if let Some(f) = pkg.datafeeds() {
        for i in 0..f.datafeeds().len() {
            let df = &f.datafeeds()[i];

            println!("Datafeed: {}", df.name);
            println!("Alias: {}", df.alias);
            println!("Active: {}", if df.active { "Yes" } else { "No" });
            println!("Status: {}", df.status);
            println!("Description: {}", df.description);
            println!("GUID: {}", df.guid);

            if detailed {
                println!("ID: {}", df.id);
                println!("Datafeed Type: {}", df.datafeed_type);
                println!(
                    "Enable Holding: {}",
                    if df.enable_holding { "Yes" } else { "No" }
                );
                println!("Creation Date: {}", df.update_information.creation_date);
                println!("Creation Login: {}", df.update_information.creation_login);
                println!("Update Date: {}", df.update_information.update_date);
                println!("Update Login: {}", df.update_information.update_login);

                println!("Schedule Parent GUID: {}", df.schedule_parent_guid);
                println!("Schedule Parent Name: {}", df.schedule_parent_name);
                println!("Schedule GUID: {}", df.schedule_guid);
                println!("Selected Target: {}", df.selected_target);
            }

            if i < f.datafeeds().len() - 1 {
                print_separator_line();
            }
        }
    }
}

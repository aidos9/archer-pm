use archer_package_manager::error::APMError;

use crate::{cli::InformationOperation, util::package_path};

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
        InformationOperation::Overview { detailed, verbose } => {
            if verbose {
                println!("Extracting ")
            }

            todo!("This mode is currently unsupported")
        }
        InformationOperation::Applications {
            list_applications,
            list_aw,
        } => todo!("This mode is currently unsupported"),
        InformationOperation::Datafeeds {
            list_all,
            list_detailed,
        } => todo!("This mode is currently unsupported"),
        InformationOperation::Notifications => todo!("This mode is currently unsupported"),
        InformationOperation::Solutions => todo!("This mode is currently unsupported"),
        InformationOperation::Dashboards => todo!("This mode is currently unsupported"),
        InformationOperation::Workspaces => todo!("This mode is currently unsupported"),
        InformationOperation::DataDrivenEvents => todo!("This mode is currently unsupported"),
        InformationOperation::Levels => todo!("This mode is currently unsupported"),
    }

    return Ok(());
}

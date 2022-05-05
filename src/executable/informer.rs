use archer_package_manager::error::APMError;

use crate::cli::InformationOperation;

pub fn execute_informer_op(op: InformationOperation) {
    match execute_op(op) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error - {}", e.error_type());
            eprintln!("Description: {}", e.description());
        }
    }
}

fn execute_op(operation: InformationOperation) -> Result<(), APMError> {
    match operation {
        InformationOperation::Overview {
            detailed,
            verbose,
            path,
            name,
        } => todo!("This mode is currently unsupported"),
        InformationOperation::Applications {
            list_applications,
            list_aw,
            path,
            name,
        } => todo!("This mode is currently unsupported"),
        InformationOperation::Datafeeds => todo!("This mode is currently unsupported"),
        InformationOperation::Notifications => todo!("This mode is currently unsupported"),
        InformationOperation::Solutions => todo!("This mode is currently unsupported"),
        InformationOperation::Dashboards => todo!("This mode is currently unsupported"),
        InformationOperation::Workspaces => todo!("This mode is currently unsupported"),
        InformationOperation::DataDrivenEvents => todo!("This mode is currently unsupported"),
        InformationOperation::Levels => todo!("This mode is currently unsupported"),
    }

    return Ok(());
}

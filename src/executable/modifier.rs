use archer_package_manager::error::{APMError, APMErrorType};
use archer_package_manager::package_compression;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use crate::cli::ModiferOperation;
use crate::util::y_n_question;

pub fn execute_modifier_op(op: ModiferOperation) {
    match execute_op(op) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error - {}", e.error_type());
            eprintln!("Description: {}", e.description());
        }
    }
}

fn execute_op(op: ModiferOperation) -> Result<(), APMError> {
    match op {
        ModiferOperation::RemoveChecksum {
            name,
            path,
            output_path,
            verbose,
        } => {
            if let Some(_name) = name {
                eprintln!("Error: Manager is not enabled.");
                exit(1);
            }

            if let Some(path) = path {
                let p = Path::new(&path);

                if !p.exists() {
                    eprintln!("Error: There is no file at {}", path);
                    exit(1);
                }

                let dest = output_path.unwrap_or(path.clone());

                if verbose {
                    println!("Files:");
                    for f in package_compression::dump_file_names_zip(&path)? {
                        println!("{}", f);
                    }
                    println!();
                }

                let (zip, found) = package_compression::remove_checksum(&path)?;

                if !found {
                    eprintln!("Checksum not found");
                    return Ok(());
                }

                write_bytes(&zip, &dest)?;

                println!("Checksum file removed");
                println!("Output: {}", dest);
            }
        }
        ModiferOperation::AddChecksum { path, output_path } => {
            let p = Path::new(&path);

            if !p.exists() {
                eprintln!("Error: There is no file at {}", path);
                exit(1);
            }

            let dest = output_path.unwrap_or(path.clone());

            if Path::new(&dest).exists() {
                if !y_n_question(&format!(
                    "There already exists a file at {}\nOverwrite?",
                    dest
                )) {
                    eprintln!("Aborting");
                    exit(0);
                }
            }

            let (zip, checksum) = package_compression::insert_checksum_zip(&path)?;

            write_bytes(&zip, &dest)?;

            println!("Checksum Added: {}", checksum);
            println!("Output: {}", dest);
        }
        ModiferOperation::MakePackage {
            add_to_db,
            input_directory,
            name,
            version,
            output_path,
        } => todo!(),
    }

    return Ok(());
}

pub fn write_bytes(b: &[u8], p: &str) -> Result<(), APMError> {
    let mut f = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .create(true)
        .open(p)
        .map_err(|e| APMErrorType::FileOpenError.into_apm_error(e.to_string()))?;

    f.write_all(b)
        .map_err(|e| APMErrorType::FileWriteError.into_apm_error(e.to_string()))?;

    return Ok(());
}

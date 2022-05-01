use archer_package_manager::error::{APMError, APMErrorType};
use archer_package_manager::{packages, zip_manipulation};
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
                    for f in packages::dump_file_names_zip(&path)? {
                        println!("{}", f);
                    }
                    println!();
                }

                let (zip, found) = packages::remove_checksum_zip(&path)?;

                if !found {
                    eprintln!("Checksum not found");
                    return Ok(());
                }

                write_bytes(&zip, &dest)?;

                println!("Checksum file removed");
                println!("Output: {}", dest);
            } else {
                eprintln!("Error: No path specified");
            }
        }
        ModiferOperation::AddChecksum {
            name,
            path,
            output_path,
            remove_checksum,
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

                if Path::new(&dest).exists() {
                    if !y_n_question(&format!(
                        "There already exists a file at {}\nOverwrite?",
                        dest
                    )) {
                        eprintln!("Aborting");
                        exit(0);
                    }
                }

                if remove_checksum {
                    println!("Removing checksum if found");
                }

                let (zip, checksum) = packages::insert_checksum_zip(&path, remove_checksum)?;

                write_bytes(&zip, &dest)?;

                println!("Checksum Added: {}", checksum);
                println!("Output: {}", dest);
            } else {
                eprintln!("Error: No path specifed.");
                exit(1);
            }
        }
        ModiferOperation::MakePackage {
            add_to_db,
            input_directory,
            name,
            version,
            output_path,
            verbose,
        } => {
            if add_to_db {
                eprintln!("Error: Manager is not enabled.");
                exit(1);
            }

            let dir_path = Path::new(&input_directory);

            if !dir_path.exists() {
                eprintln!("Error: Could not find a directory at {}", input_directory);
                exit(1);
            } else if !dir_path.is_dir() {
                eprintln!("Error: There was no directory found at {}", input_directory);
                exit(1);
            }

            let dest;

            if let Some(output_path) = output_path {
                dest = output_path;
            } else if let Some(name) = name {
                if let Some(version) = version {
                    dest = format!("{}-v{}.zip", name, version);
                } else {
                    dest = format!("{}.zip", name);
                }
            } else {
                if let Some(Some(last_component)) =
                    dir_path.components().last().map(|v| v.as_os_str().to_str())
                {
                    dest = format!("{}.zip", last_component);
                } else {
                    dest = "output.zip".to_string();
                }
            }

            println!("Compressing...");

            create_package_file(&input_directory, &dest, verbose)?;

            println!("Successfully created package");
            println!("Output: {}", dest);
        }
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

pub fn create_package_file(dir: &str, out: &str, verbose: bool) -> Result<(), APMError> {
    let (zip_contents, files) = zip_manipulation::compress_directory(dir, verbose)?;

    if let Some(files) = files {
        for f in files {
            println!("Compressed: {}", f);
        }
    }

    let (zip_contents, checksum) = packages::add_checksum_zip(zip_contents)?;

    println!("Checksum: {}", checksum);

    return write_bytes(&zip_contents, out);
}

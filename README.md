# archer-pm
RSA Archer Package Manager, is a project intended to create a better method of managing RSA Archer packages.

# Currently Supported Features
- [ ] Local Package Management
- [ ] Remote Package Management
- [ ] Upload/Download/Install to Archer Instance Support
- [x] Compress directories into packages
- [x] Add RSA archer compatible checksums to modified packages
- [x] Remove the checksum from a zipped package


# Basic Usage
All subcommands support the '-h' option which will show help messages and the available options. e.g. ```apm -h``` or ```apm mod -h``` or ```apm mod -r -h```

### Compressing a directory into a package
To create a package from a directory please specify the directory path, this option will automatically calculate the checksum.
<b>NOTE:</b> No validation is performed as to whether a valid package's contents will exist in this directory, so ensure that the directory contains a valid package's files.

This example compresses a directory called 'package_dir' into a package file called 'package.zip'
```
apm mod make-package -o package.zip package_dir 
```

For more options specify -h:
```
apm mod -m -h
```

### Adding a checksum (to a zip)
Before adding a checksum to a package please ensure it has been removed already, if the checksum is already there, the program will notify you and exit. 

The below code is an example of adding a checksum to a package and the outputting to 'checksum_package.zip'.
```
apm mod add-checksum -p package.zip -o checksum_package.zip 
```

If you wish to remove the old checksum as a part of adding the new checksum specify the -r option. This will be slower than simply running the above command.
```
apm mod -a -r -p package.zip -o checksum_package.zip
```

For more options specify -h:
```
apm mod -a -h
```

### Removing a checksum (from a zip)
RSA Archer uses a hidden file to store the hash for a package. Before adding a checksum you must remove the existing one. This command will take a file called 'package.zip', extract it in memory and copy all the files excluding the hidden one to a new zip file and output it to the desired path. If no output path is specified the original file will be overridden.
```
apm mod remove-checksum -p package.zip
```

This can also be written like so:
```
apm mod -r -p package.zip -o package.zip
```

For more options specify -h:
```
apm mod -r -h
```

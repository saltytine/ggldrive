# Basic usage

After installing `ggldrive` and creating an API token, you can add it by running
```
ggldrive account add
```
which will prompt you for your created user id and secret.

## List all files
You should be able to list all files in your Google Drive by running
```
ggldrive files list
```
and show info on a particular file with
```
ggldrive files info <file id>
```
where `<file id>` is the id/hash(?) printed by `ggldrive files list`.

## Downloading files
A single file can be downloaded with
```
ggldrive files download <file id>
```
however, this will only work for files with "binary content" and not Google Docs files. To download files that can be edited with Google Docs it's necessary to use the `ggldrive export <file id> <output file>` command.

## Downloading directories
Downloading a directory is done by the same command as downloading a file, but with the `--recursive` flag:
```
ggldrive files download --recursive <file id (directory)>
```
this will create a directory with the same name as the Google Drive directory and copy all files recursively. It may take a while before you get any output, but you should get a status on which directories and files are created and downloaded. 

Note: any Google Docs files will be silently ignored when doing this.
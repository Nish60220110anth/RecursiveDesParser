# Compiler Assignment 

## **Name** Nishanth 
## **Date** Nov 17 2022

### About 
Recursive descent parser in rust

### Problem (in assignment)
-----
> *A  -> B C*
> 
> *B  -> B! | D*
> 
> *C  -> *A | epsilon*
> 
> *D  -> n  | (A)*
----
> After Resolving left recursive conflict 
> 
> *A  -> B C*
> 
> *B  -> D B`*
> 
> 
> *B' -> !B' | epsilon*
> 
> *C  -> *A | epsilon*
> 
> *D  -> n  | (A)*
----

### Supported Plaforms
> Only windows and Unix variants (ex. Mac,Unix) supported .
>
> For non supported platforms , output is produced but not in correct format 

### Why some platforms not supported ?
> Input for the program is taken from the file which may be created in any os , so they have different chars to represent newline.
>
> Internally program removes every new line character (newline is allowed for readability).
>
> But if input doesn't contain any newline, then this program has not restriction

### For rust developers
> `production` profile is used for low binary size

### How to Run (if rust installed)
1. Clone the repo
2. Run `cargo run --profile=production -- <input_file_path>`

### How to Run (no rust installed)
1. Download the binary release.
2. Run `<binary_release_name> .\<input_file_path>`

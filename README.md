# prgf

## About 
Prgf aims to be an easy to modify command line utility.

It allows users to create "bog standered" file of any kind.         
Simply add the name of the file type, the extension and the contents             
that should be wrote to the file upon create into a file called `prgf_langs.txt` in your                
system of choices config directory.

## Setup
prgf should be installed from `crates.io` or using `cargo`.

Once it is installed, there is an file called `example.txt` inside.                   
This shows the general layout for how to configure and includes config for 4 languages:

- c 
- cpp
- java
- rust

## Adding more languages
To add more languages to prgf create/modify the `prgf_langs` file        
in your systems config directory and follow these guidlines.

### Declaring language name and extension
```
// language_name .extension
```

The name an extension should be in line with the //.

### Declaring contents
```
// langauge_name .extension
bellow the line starting with // name .extension  
you should put anything that should be in the file during creation
```

If the line bellow this is empty add the following:
```
//
```

An empty line with // indicates the end of the file.

> [!WARNING]
> because prgf uses // to determine when a files info is starting
> and when it ends, your file cannot contain information that has // in it.
>
> This will possibly be changed if i dont abondon the project here

## Example
Here is a basic example of how to configure prfg to make a basic python            
file:

```
// python .py
if __name__ == __main__:
    print("Get programming.\n")
```

## Footnote
This is the first project i have created in the rust       
programming language and i would love any feed back on how to make      
my code more *rustic*.

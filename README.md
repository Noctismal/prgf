# prgf

> [!WARNING]
> prgf needs a file called prgf_langs.txt with configuration data in .confg/prgf 
> to work, there is a template to get started that includes formatting for
> c, cpp, java and rust in this repo called the same.

## About
Prgf aims to be an easiy to modify command line tool for creating basic        
programming files.

"What is a basic programming file?"

A basic programming file in this sense means a file filled with all the boiler plate                
needed already in it so you could call prgf and run the main file it created and have it 
print text to the screen!

## Modifying to add more languages

Where the program lives there should be a file called `prgf_langs.txt`               
inside this file add the following (replacing lang and extension with the deserived 
```
// name extension
```

And then bellow add the content that should be in the file.

If what you have added is at the end of the file make sure that you add // to the line bellow                 
the content.

### Python example
```
// python .py
if __name__ == __main__:
    print("Get programming\n")
//
```

## Example

To make a basic c file enter this in the terminal once the program is added to your path

```
$ prgf -f c 
```

This will give you a file called `main.c` that looks like this

```c 
#include <stdio.h>

int main(void) {
    printf("Test\n");

    return 0;
}
```

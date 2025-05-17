# prgf

> [!IMPORTANT]
> prgf comes pre installed with formatting  and extensions for c, cpp, rust, java.
> To include more add the formatting and extension to use in prgf_langs.txt

Prgf aims to be an easiy to modify command line tool for creating basic        
programming files.

"What is a basic programming file?"

A basic programming file in this sense means a file filled with all the boiler plate                
needed already in it so you could call prgf and run the main file it created and have it 
print text to the screen!

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

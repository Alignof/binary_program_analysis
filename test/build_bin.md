## source code
```c
#include <stdio.h>
int main(void) {
    printf("Hello World!\n");
    return 0;
}
```

## Elf32
```sh
$ gcc -m32 test.c -o Elf32
$ file Elf32
Elf32: ELF 32-bit LSB pie executable, Intel 80386, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux.so.2, BuildID[sha1]=d73d15c492b38333d2cd45a5fd4bbe7ea0c77ff7, for GNU/Linux 4.4.0, not stripped
```

## Elf64
```sh
$ gcc test.c -o Elf64
$ file Elf64
Elf64: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=1e50e69370dad85d9c18943fdc7c79fa02a0aefe, for GNU/Linux 4.4.0, with debug_info, not stripped
```

## Pe32
```sh
$ i686-w64-mingw32-gcc test.c -o Pe32
$ file Pe32.exe
Pe32.exe: PE32 executable (console) Intel 80386, for MS Windows, 18 sections
```

## Pe64
```sh
```


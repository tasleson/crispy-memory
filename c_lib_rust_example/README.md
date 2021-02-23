### Source code for the blog post:

[How-to: Writing a C shared library in rust](http://blog.asleson.org/2021/02/23/how-to-writing-a-c-shared-library-in-rust/)

To build and run
```
$ cargo build
$ gcc -Wall -g -O0 main.c -I. -Ltarget/debug/ -lsomelibname
$ LD_LIBRARY_PATH=target/debug ./a.out
```

<link href="/style/style.css" rel="stylesheet"/>
<include "header.html">

# Hello World
This direct use of the `write` syscall. It takes three arguments: 
the file descriptor, the buffer, and the length of the buffer. 
A file descriptor of `1` represents the `stdout`.  
```
:arch x86_64 linux

entry: *write(1, "Hello, World!\n", 14)
```


# Fibonacci
The example assumes the binary is linked with `libc`.
`extern` is used to declare the `scanf` and `printf` functions.
```
:arch x86_64 linux
extern scanf [1:], ? @varg
extern printf [1:], ? @varg

fibonacci 4 n, -> 4 {
   (n <= 1): ret n
   ret !fibonacci (n - 1) + !fibonacci (n - 2)
}

entry {
   %terms 4
   $scanf "%d", &terms

   loop (%i 4 ; 'i ++ ; i < terms):
      $printf("%d\n", !fibonacci i)
}
```


# Bubble Sort
```
:arch x86_64 linux

:verb run /bin/sh {
   sharc #FILE
   chmod +x #NAME
   ./#NAME
}

entry {
   %array 2:? <- {2, 8, 9, 7, 4, 3, 6, 5, 1, 0}

   !bubble_sort(array, @len array)

   // print the array
   loop (%i 4 ; 'i ++ ; i = @len array):
      $printf("%d\n", array.i)
}

bubble_sort 2:?, 4 len:
   loop (%i 4 ; 'i ++ ; i < len):
      loop (%j 4 ; 'j ++ ; j < len-i-1):
         (array.j > array.j + 1) {
            %temp array.j
            'array.j <- array.j + 1
            'array.j <- temp
         }
```


# Fat Pointers
The `#WORD` macro resolves to the word size of the architecture.  
`T -> str` allows us to require any type `T` as long as it can be cast to the type `str`.
```
:arch x86_64 linux

struct str {
   #WORD len,
   [1:]  ptr,
}

// string must be null terminated
cast [1:] string -> str {
   %string <- [1:] @cast string
   %i #WORD
   loop ('i ++ ; string.i = 0)
   ret {string, i}
}

print T -> str s {
    %s <- s -> str
    *write(1, s.ptr, s.len)
}

entry: !print "Hello, World!"
```


# Iterators
This is how a possible iterator implementation could be done.  
Structs can take generic types
```
:arch x86_64 linux

extern malloc #WORD -> []
extern free []
extern memcpy [1:], [1:], #WORD -> []

struct Iterator[T] {
   #WORD len,
   [T:]  ptr,
}

from T: array -> Iterator[T] {
   %heap [T:] <- $malloc @size array
   $memcpy(heap, array, @size array)
   ret {@len array, heap}
}

map Iterator[T] iter, [] func -> Iterator[T] {
   loop (%i 4 ; 'i ++ ; i < iter.len):
      !func iter.ptr.i
   ret iter
}

for_each Iterator[T] iter, [] func {
   loop (%i 4 ; 'i ++ ; i < iter.len):
      !func iter.ptr.i
   $free iter.ptr
}

entry:
   Iterator[4] !from {1, 2, 3, 4, 5, 6, 7, 8}
      => !map |x: x * 2|
      => !for_each |x: $printf("%d\n", x)|
```

<include "footer.html">

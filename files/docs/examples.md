<link href="/style/style.css" rel="stylesheet"/>
<include "header.html">

Most of these assume `libc` is linked with the binary.

# Hello World
`extern` is used to declare the `puts` libc function.
using `_start` just to make it more minimal. (libc `main` would require us to return a val)
```
extern fn puts(str: *[u8]) u32;

fn _start():
	$puts "Hello, World!\0";
```


# Fibonacci
**Shard** doesn't have a way to deal with variadic funcs yet, so we have to
*cheat* a little by defining `printf` and `scanf` with fixed arguments.
```
extern fn printf(str: *[u8], i: u32) u32;
extern fn scanf(str: *[u8], i: *[u32]) u32;

fn fibonacci(n: u32) u32 {
   if n <= 1: ret n;
   $fibonacci(n - 1) + $fibonacci(n - 2)
}

fn main() u32 {
	// forced raii for now, we'll prob add a way to uninit
   let terms: u32 = 0;
   $scanf("%d", &terms);

	loop let i = 0 {
		if i == terms: break;
		$printf("%d\n", $fibonacci(i));
		i + 1
	}

	ret 0;
}
```


# Bubble Sort
`&` signifies a fat pointer, which is basically `(*T, usize)` but with builtins to make it easier to work with.  
```
use core::fat::meta as len;

fn main() u32 {
	let array: &mut [u32] = {2, 8, 9, 7, 4, 3, 6, 5, 1, 0};

   $bubble_sort array;

   // print the array
	loop let i = 0 {
		if i == $len array: break;
		$printf("%d\n", array[i]);
		i + 1
	}

	ret 0;
}

fn bubble_sort(array: &mut [u32]) {
	loop let i = 0 {
		if i == $len array: break;
		loop let j = 0 {
			if j == $len array - i - 1: break;
			if array[j] > array[j + 1] {
				let temp = array[j];
				array[j] = array[j + 1];
				array[j] = temp;
			}
			j + 1
		}
		i + 1
	}
}
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

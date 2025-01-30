<link href="/style/style.css" rel="stylesheet"/>
<include "header.html">

Most of these assume `libc` is linked with the binary.

# Hello World
`extern` is used to declare the `puts` libc function.
using `_start` just to make it more minimal. (libc `main` would require us to return a val)
```
let puts = extern "puts" |str: *[u8]| u32;

export "_start" ||:
	puts("Hello, World!\0");
```


# Fibonacci
**Shard** doesn't have a way to deal with variadic funcs yet, so we have to
cheat a *little* by defining `printf` and `scanf` with fixed arguments.  
`recur` is used to so we can recursively call the func we're in.
```
let printf = extern "printf" |str: *[u8], i: u32| u32;
let scanf = extern "scanf" |str: *[u8], i: *[u32]| u32;

let fibonacci = |n: u32| u32 {
   if n <= 1: ret n;
   recur(n - 1) + recur(n - 2)
};

export "main" || u32 {
   let terms: u32 = 0;
   scanf("%d\0", &terms);

	loop let i = 0 {
		if i == terms: break;
		printf("%d\n\0", fibonacci(i));
		i + 1
	}

	0
}
```


# Bubble Sort
`&` signifies a fat pointer, which is basically `(*T, usize)` but with builtins to make it easier to work with.  
```
use core::fat;

let bubble_sort = |array: &mut [u32]| {
	loop let i = 0 {
		if i == fat::meta(array): break;
		loop let j = 0 {
			if j == fat::meta(array) - i - 1: break;
			if array[j] > array[j + 1] {
				let temp = array[j];
				array[j] = array[j + 1];
				array[j] = temp;
			}
			j + 1
		}
		i + 1
	}
};

export "main" ||: u32 {
	let array: &mut [u32] = [2, 8, 9, 7, 4, 3, 6, 5, 1, 0];

   bubble_sort(array);

   // print the array
	loop let i = 0 {
		if i == fat::meta(array): break;
		printf("%d\n\0", array[i]);
		i + 1
	}

	0
}
```

<include "footer.html">

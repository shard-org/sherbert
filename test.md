# Iterators
A fairly lengthy example on how an iterator implementation could be done.

```
:arch x86_64 linux
:linker #LINKER -lc

extern malloc 8 -> []
extern free []

struct Iterator {
   4    len,
   [1:] array,
}

destr Iterator iter {
   $free #iter.array
}

from {1:} heap -> Iterator {
   %array = $malloc @size heap
   'array = heap
   ret {@size heap, array}
}

map Iterator iter, [] func, -> Iterator {
   loop (%i 4 ; 'i ++ ; i < iter.len):
      'iter.array.i = !func iter.array.i
   ret iter
}

for_each Iterator iter, [] func {
   loop (%i 4 ; 'i ++ ; i < iter.len):
      !func iter.array.i
   @destr iter
}


entry:
   Iterator !from {1, 2, 3, 4, 5, 6, 7, 8}
      => !map |x: x * 2|
      => !for_each |x: $printf("%d\n", x)|
   ret
```

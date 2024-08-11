```
:NAME vec
:LIB libc

extern malloc  #WORD -> []
extern memcpy  [], [], #WORD -> 4
extern realloc [], #WORD -> []
extern printf  [1:], @varargs ?
 
struct Vec<T> {
   [T:]  contents
   #WORD len
   #WORD cap
}

from T:? array -> Vec<T> {
   %contents [T:] = $malloc @size array
   $memcpy contents, array, @size array
   ret {contents, @len array, @len array}
}

new -> Vec<T>:
   ret {0, 0, 0}

destr Vec<T> {
   $free #1->contents
}

op Vec<T> . ? -> [T] {
   #1->contents.#2
}

push Vec<T> vec, T element {
   (vec->contents = 0) {
      'vec->contents = $malloc (@size T * 2)
      'vec->cap = 2
   }

   (vec->len = vec->cap) {
      'vec->cap * 2
      'vec->contents = $realloc vec->contents, (@size T * vec->cap)
   }

   'vec->contents.(vec->len) = element
   'vec->len = vec->len + 1
}

entry {
   %vec = Vec<4> !new

   vec => !push 1
   vec => !push 2
   vec => !push 3
   vec => !push 4
   vec => !push 5

   $printf "third num: %d\n", vec.2
   @destr vec
}
```

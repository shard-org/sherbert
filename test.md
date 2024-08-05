> ```
> :ARCH x86_64 linux
> :LINKER #LINKER -lc
> 
> :EXTERN malloc 8 -> []
> :EXTERN free []
> 
> :STRUCT Iterator {
>    4 len
>    [1:] array
> }
> 
> from {1:} heap -> Iterator {
>    %array = $malloc @size heap
>    '[array] = heap
>    ret {@size heap, array}
> }
> 
> map Iterator iter, [] func, -> Iterator {
>    loop, init {%i 4}, ('i ++ & i < iter->len):
>       'iter->array.i = !func iter->array.i
>    ret iter
> }
> 
> for_each Iterator iter, [] func {
>    loop, init {%i 4}, ('i ++ & i < iter->len):
>       !func iter->array.i
>    $free iter->array
> }
> 
> entry {
>    Iterator !from {1, 2, 3, 4, 5, 6, 7, 8}
>       => !map |x: x * 2|
>       => !for_each |x: $printf "%d\n", x|
> }
> ```

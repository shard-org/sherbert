<link href="style.css" rel="stylesheet"/>
<include "header.html">

**Shard** - "The OSR of programming languages"

<div class=text>

## Premise
Flexible, terse, and non-opinionated assembly-inspired programming language 
geared toward compile target flexibility and direct control.  
No assumptions, everything must be explicitly declared, including the code emitted.

```
// not your average language :v
average [T:] nums, #WORD len, -> T {
   %sum T 
   %i #WORD
   loop ('i ++ ; i < len):
      'sum + nums.i
   ret sum / len
}
```


## Features
<div class=block>

\- Insane compiletime functionality.. the code emitted is defined at comptime after all.. Think of **Shard** as a really fancy text preprocessor.  
\- Basically every part of the language is customizable.  
\- No <i>"Safety Features"</i> preventing you from doing stupid stuff.  
\- Architecture details are defined within the standard library, meaning adding support for additional ones doesn't
  require modifying any compiler source code, and can by done by the user. As long as there's a compatible assembler, of course.  
\- Shark mascot! (best of all the features)  
</div>

</div>

```
/* Linear allocatior (first fit). */

// 1 means occupied, 0 means block is free. 
// BLOCK_TABLE is a bit representation of all addressable blocks
BLOCK_TABLE: #WORD
:macro BLOCK_SIZE 1024

malloc #WORD size -> {[], #WORD} {
    %n_blocks = size / BLOCK_SIZE 

    %start_zeros = 0
    %n_zeros = 0
    l1 loop ('start_zeros++; start_zeros < (@size BLOCK_TABLE * 8)) {
        ((BLOCK_TABLE >> start_zeros && 1) == 1)
            jmp l1;
        // found zero, free chunk ahead
        loop ('n_zeros++; (BLOCK_TABLE >> start_zeros && 1) ~= 1)

        (n_zeros >= n_blocks): 
            //success! we found a large enough contiguous span equal or larger
            // than what the caller requested.
            ret { start_zeros * BLOCK_SIZE, n_zeros }

        // not found. we will move to the next free blocks
    }
    // our bit cursor moved past the maximum size of blocks.
    ret { 0, 0 }
}

free T -> {[], #WORD} fat_ptr {
    %cleared = 0; // copy

    loop ('cleared++; cleared < fat_ptr.1):
        'BLOCK_TABLE ^^ BLOCK_TABLE && (1 << cleared)
}

entry {
    %string [1:] = !malloc 14

    // Null pointer check
    (~string):
        $printf ("Unable to allocate %d bytes!", 14)

    '[string] = "Hello, World!\0"

    $printf ("%s", string)
    !free string
}
```

<div>
<include "footer.html">

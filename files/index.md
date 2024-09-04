<link href="style/style.css" rel="stylesheet"/>
<include "header.html">

<div style="position:relative;display:inline-block;">
<img src="shark.png" style="position:absolute;"/>
<div style="padding-left:9px;">

**Shard** - "The OSR of programming languages" *~~is this even a good quote?~~*

</div>
</div>

## Premise
Flexible, terse, no compromise, and non-opinionated assembly-inspired programming language 
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

\- Insane compiletime functionality; The code emitted is defined at comptime after all.. Think of **Shard** as a really fancy text preprocessor.  

\- Basically every part of the language is customizable.  

\- No <i>"Safety Features"</i> preventing you from doing stupid stuff. :L  

\- Architecture details are defined within the standard library, meaning adding support for additional ones doesn't
   require modifying any compiler source code, and can by done by the user. As long as there's a compatible assembler, of course.  

\- Self contained projects; everything is defined in a single file. (doesn't mean you can't include additional)  

\- Shark mascot! (best of all the features) ~~if you wanna draw a full version lmk~~

</div>


## Non-Features
<div class=block>

\- Shard is not meant to be cross-platform or an IR. (although you could certainly try :>)  
   All architectures are inherently dissimilar, and reconsiling them cannot be done without compromises.  

\- Optimizations are fully left to the user (or in part to whoever writes the standard library). 
   If you need crazy fast code use 
   [C](https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html),
   [Rust](https://www.rust-lang.org/), or
   [Zig](https://ziglang.org/)
   instead (or just git gewd)  

\- Standard libraries are for defining the architecture only. They don't provide any additional functionality.
   Use [libc](https://musl.libc.org/) if you want an expansive standard library.

</div>


## FAQ
<div class=block>

\- Yes, there is a difference between *"compile target flexibility"* and *"cross-platform"*.  
   The first is about being able to write **Shard** for any architecture, 
   the latter is about compiling the same code to multiple architectures.

\- When are you gonna release? ... :( Yeah we're kinda starved for people to work on it..  
   Please help out if you can! (join the [Discord](https://discord.gg/f5FVgr7gxX) for more info)


</div>
<include "footer.html">

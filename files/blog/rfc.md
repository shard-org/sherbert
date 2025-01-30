<link href="/style/style.css" rel="stylesheet"/>
<include "header.html">

## On the Hyperpreprocessor

RFC 1; 24-10-2024  

By now shard has become basically a really fancy text preprocessor.
It takes in a file, interprets it based on a config, and spits out another.
(I'm aware this is basically what all compilers do, but ye know what I mean)  

Currently shard is in a weird limbo of having a lot of builtin features making it a simple language,
while also fully relying on the preprocessor to do anything remotely useful.  

We should take a step back, and a step forward. Think shard should become a kind of *meta language*
(not talking about you ML haskell people :v) containing only the most basic of features; 
both to not intrude on the preprocessor, and to allow for the most flexibility in the emitted code.
This change would offload a lot of work from us, and let us polish the preprocessor to make it very easy to use.
We should also consider moving more though towards the type system, making it assumption-less, and more adaptable.  


## Limit the scope of operations

Currently shard has a lot of builtin features, such as loops, 
diff operator semantics, extern functions, &c.  
These are (in the grand scheme of things), irrelevant.

I propose we limit the scope to the following:
- **symbols:** these need to be accessible to the end user,
    the interpreter needs to cover handling, passing, manipulation, and resolution.
- **types:** the core building block, making shard easily adaptable as a language,
    good type mechanics need to include definition, matching, and conversions.
    (this group includes all abstractions to group, and resolve types like generics)
- **labels:** blocks, functions, etc can be all destilled to this base block,
    users need to be able to register, call, and manipulate these. (attributes, scopes, and bodies)
- **builtins:** a way to directly invoke the interpreter's functionality, and define its behaviour.
- **tags:** a way to add metadata to objects (not oop, just any compiler construct).
- **config:** this should still remain within the language, these are general project configs,
    processing rules, and other things not directly related to code (scripts, etc).


## Interpreter?

With these changes the compiler would behave more like an interpreter,
evaluating code at compiletime, it's just an unfortunate side effect that some assembly is emitted.  

It would handle all the heavy lifting outlined above, together with calling scripts, and making coffee.
This point of view greatly simplifies the language, and makes it insanely powerful.


## Current compiler changes

the parser needs to be reworked to make the tree based on the interpreted language,
and only check for the bare minimum (making sure symbols are well formed, etc).

The remaining bits will remain basically the same. The current lexer is already sufficient.

<include "footer.html">

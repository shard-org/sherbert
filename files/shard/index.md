<link href="style/style.css" rel="stylesheet"/>
<include "header.html">

<div style="position:relative;display:inline-block;">
<img src="shard/shark.png" style="position:absolute;"/>
<div style="padding-left:9px;">

**Shard** - "The OSR of programming languages" *~~is this even a good quote?~~*

</div>
</div>

## Premise
Simple, pragmatic, and non-opinionated functional flavoured imperative language.  
<!-- more cool things -->

```
// pretty average language :p
let average = |a: *[u32]|: u32 {
	loop let (sum, i) = (0, 0) {
		if i == a.len: break sum;
		(sum + a[i], i + 1)
	} / a.len
};
```

## Features
- All the things you'd expect in a modern lang like: generic types, fat ptrs, etc.
- Explicit mutability, nullability and fat pointers.
- Clojure like threading macros, allowing for a method-chain like syntax.  
  (I cannot believe *ALL* languages dont have this, its just so useful)
- Compiling to llvm IR, making it decently fast and platform agnostic out of the box.
- Linear type system and minimal move semantics.
- Pattern matching, and destructuring. Pretty much Rust with few confusing syntaxes removed.
- Shark mascot! (best of all the features) ~~if you wanna draw a full version lmk~~

## Non-Features
- We arent planning on making a package manager or build system for shard.  
  Although this might change in the future if the need arises.

## FAQ
- When are you gonna release? ... :( Yeah we're kinda starved for people to work on it..  
  Please help out if you can! (join the [Discord](https://discord.gg/f5FVgr7gxX) for more info)
- Where's the old shard? This is a new project. I realized the old one was.. not *very* useful.
  Don't get me wrong, I'd love to see it done in the future, but I decided to take a more pragmatic direction towards the dev.
  The previous shard wasnt goin anywhere cause of disagreements on the direction, bikeshedding, and all those other fun things.  
  One day we might also make **Phase** as a companion project, but thats a long way off.
- Who's working on this? Check [the repo](https://github.com/shard-org/sharc) for up to date info.  
  Currently its mostly me + bullying others into helping. 


<include "footer.html">

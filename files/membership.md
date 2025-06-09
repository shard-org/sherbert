<link href="style/style.css" rel="stylesheet"/>
<include "header.html">

## Membership
Membership is _free_ ~as in libre~ -- it will only cost you some effort.
There is no regular standard application process, by joining and actively
being a part of the community, your willingness to join is _implicit_,
and its more valuable than filling out a silly form. If you are a part of
the community, and we get the right kind of vibes, _we_ will extend you an
invitation to officially join us.

Membership means you get a few cool perks: namely,
you get to post to mailing lists without post review
(see: [<nobr>/Community</nobr>](/community) for details regarding mailing lists).
You also get a brand new, shiny `@nh3.dev` email with a username of your
choice, as well as push access to our git repositories, without having
to push ever change via the `patch` list. Of course, when you deem it
necessary, such as when making large changes, you should still send it in
via the `patch` list for us to all take a peek and discuss.

## Git

As a member, we also allow you access to both `nh3/Repo` and `CREATOR/Repo`.
Just make sure to send us a `USERNAME.pub` (usually your `~/.ssh/id_rsa`
or `~/.ssh/id_ed25519`) and send it to <patch+owner@nh3.dev>, I'll use the
username you tell me, but consider that it has to be URL friendly as its
also used for your git website at [git.nh3.dev/USER/](https://git.nh3.dev).
If you want us in on your project, make it under `nh3/` and we will let you
and anyone else you want merge patches to it. To check your git permissions,
run:
```bash
$ ssh -T git@nh3.dev
hello USERNAME

 R W C CREATOR/[a-zA-Z0-9_]*
 R W C CREATOR/private/[a-zA-Z0-9_]*
 R W C nh3/[a-zA-Z0-9_]*
 ...
```

<include "footer.html">

<link href="style/style.css" rel="stylesheet"/>
<include "header.html">

Most people prefer to join the [Discord](https://discord.gg/f5FVgr7gxX), and it is where
most ~unserious~ development and discussions occur.

<!--
however note that actual changes and technical discussions generally occur via the mailing lists.  
Either way, we welcome you in with open arms!

## Mailing lists

There are currently two mailing lists: `patch` and `dev`. The `patch` mailing list
is for the sending and discussion of patches and contributions. The purpouse of
`dev` is for anything else: bug reports, feature requests, general discussion on the
usage of software ect.

Note that, unlike the [Discord](https://discord.gg/f5FVgr7gxX), mailing lists are
moderated (to prevent things like spamming all of our emails). This means that,
until you have are an NH3 member, your posts will be individually reviewed before
reaching the list users. We hope to be quick in this, but our community is mostly
european so expect longer delays the further you are from our timezone.
For more information regarding membership, see the [/membership](/membership) page.

Both `patch` and `dev` are configured to respond to certain commands which
you can send (e.g `patch+help@nh3.dev`, the subject and content is irrelevant).
A description of these commands can be found below:

| Suffix                        | Function |
| ----------------------------- | -------- |
| `+subscribe@nh3.dev`          | Subscribe to the mailing list (read/write) |
| `+subscribe-digest@nh3.dev`   | Subscribe to the digest version of the mailing list (read/write) |
| `+subscribe-nomail@nh3.dev`   | Subscribe without receiving e-mails from the mailing list (write) |
| `+unsubscribe@nh3.dev`        | Unsubscribe from the mailing list |
| `+unsubscribe-digest@nh3.dev` | Unsubscribe from the digest version |
| `+unsubscribe-nomail@nh3.dev` | Unsubscribe from the nomail version |
| `+get-N@nh3.dev`              | Retrieve message number N |
| `+help@nh3.dev`               | Recieve a help message containing this description and list-specific details |
| `+faq@nh3.dev` (`patch` only) | Recieve a guide on how to post in `patch` |

## Git

As a member, we also allow you access to both `nh3/Repo` and `CREATOR/Repo`.
Just make sure to send us a `USERNAME.pub` (usually your `~/.ssh/id_rsa`
or `~/.ssh/id_ed25519`) to <patch+owner@nh3.dev>, We'll use the
username you tell us, however remember that it should be URL friendly as its
also used for your git website at [git.nh3.dev/USER/](https://git.nh3.dev).
If you want us in on your project, make it under `nh3/` and we will let you
and anyone else you want merge patches to it. To check your git permissions, run:
```bash
$ ssh -T git@nh3.dev
hello USERNAME

 R W C CREATOR/[a-zA-Z0-9_]*
 R W C CREATOR/private/[a-zA-Z0-9_]*
 R W C nh3/[a-zA-Z0-9_]*
 ...
```
-->

<include "footer.html">

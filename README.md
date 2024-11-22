# git-args

Sometimes you want to always pass some arguments to git, for example `git clone
--recursive ...`. Instead of using aliases, you can use `git-args` to pass these
defaults.

`git-args` installs a new `git` executable.

You must configure the following env variables:

- `GIT_REAL`: path to the real git -- if you use Nix, this is done
automatically.
- `GIT_<SUBCOMMAND>_ARGS`: arguments that will be injected into some
subcommand. For example, `GIT_CLONE_FLAGS="--recursive --filter=blob:none"`

### What's this?

This is a script to get the current working directory of the most recently focused Alacritty window, in niri.

### How do I use it?

First install [Rust](https://rust-lang.org/tools/install/), and then run:
```
$ cargo install niri-alacritty-recent-cwd
```

Then in `~/.config/niri/config.kdl`, bind it to something like:
```
binds {
    Mod+T { spawn-sh "alacritty --working-directory $(niri-alacritty-recent-cwd)"; }
}
```

When you use this to open a new window, it will have the same working directory as the previous one.

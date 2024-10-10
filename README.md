# mavlink-hacking
Project to hack on the mavlink protocol.

## Get Services Running...

```bash
# Allow X Connections
$ xhost +local:root

# Connect to existing container
$ docker exec -it px4_mavlink_hacking /bin/bash

# Inside the px4_mavlink_hacking container, CD to home directory
$ cd ~

# Load tmuxp session (again, inside the px4_mavlink_hacking container)
# This will start a tmux session with all the tools we need
$ tmuxp load tmux_session.yaml

# In another terminal, run this project...
$ cargo run
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

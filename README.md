# EVM Observer

What does the world computer cost?

# Alert: Work-In-Progress

Not finished. Not for prime time. Not a project to bring home and meet your parents.

# Components

## `evmextract`

Dumps EVM per-instruction counts and gas consumption. Calls geth's RPC methods via the 
unix domain socket IPC interface.

Usage:
```
$ evmextract STARTING_BLOCK PATH_TO_IPC_SOCKET
```

Where:
* `STARTING_BLOCK` - the Ethereum block # to begin with
* `PATH_TO_IPC_SOCKET` - fully qualified path to geth's IPC socket, usually `$HOME/.ethereum/geth.ipc`

# Copyright and License

Copyright 2018 int08h LLC. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.


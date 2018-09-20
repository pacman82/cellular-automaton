Cellular Automaton
==================

A command line tool which evolves the state of simple 1-dimensional cellular automatons and prints
them to standard out.

Produces pretty output like this:

```
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░░██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░███░░░░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░██░░█░░░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░████░██░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░░██░█░░░░█░████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░███░░██░░██░█░░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░██░░███░███░░██░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░███░░░█░░███░░█░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░░██░█░░█░█████░░███████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░███░░████░█░░░░███░░░░░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░██░░███░░░░██░░██░░█░░░░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░███░░█░░██░███░████░░██░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░░██░█░░██████░░█░░░█░░░███░████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░███░░████░░░░░████░███░██░░░█░░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░██░░███░░░█░░░██░░░░█░░░█░█░███░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░███░░█░███░██░█░░███░██░█░█░░░█░░█░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░██░████░░██░█░░███░█░░░█░░████░░░█░░█░██░██████░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░███░░████░░░██░█████░░░█░█████░█░░█░░░░░█░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░██░████░██░░███░░░█░██░░█░░░░█░██░█░░░░░█████░░░███░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░███░░█░██░█░████░░██░█░░██░░░██░░░░█░██░░█░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░██░████░░██░█░░███░█░░█░█░░░███░░████░█░██░█░░██░█░████░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░██░░█░░░███░░████░░░████░██░██░░███░░░░█░█░░████░░█░█░░░█░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░██░████░██░░███░░░█░██░░░░█░░█░███░░█░░██░████░░░███░██░███░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░██░░█░░░░█░███░░█░██░█░█░░█████░█░░██████░░█░░░█░██░░░█░░█░░█░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░██░████░░██░█░░███░█░░█░████░░░░░████░░░░░████░██░█░█░█████████░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░██░░█░░░███░░████░░░████░█░░░█░░░██░░░█░░░██░░░░█░░█░█░█░░░░░░░░█░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░██░████░██░░███░░░█░██░░░░██░███░██░█░███░██░█░░█████░█░██░░░░░░███░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░██░░█░░░░█░███░░█░██░█░█░░██░░█░░░█░░█░█░░░█░░████░░░░░█░█░█░░░░██░░█░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░██░████░░██░█░░███░█░░█░████░████░█████░██░█████░░░█░░░██░█░██░░██░████░░░░░░░░░░░░░░
░░░░░░░░░░░░░░██░░█░░░███░░████░░░████░█░░░░█░░░░█░░░░░█░░█░░░░█░███░██░░█░█░███░░█░░░█░░░░░░░░░░░░░
░░░░░░░░░░░░░██░████░██░░███░░░█░██░░░░██░░███░░███░░░██████░░██░█░░░█░███░█░█░░████░███░░░░░░░░░░░░
░░░░░░░░░░░░██░░█░░░░█░███░░█░██░█░█░░██░███░░███░░█░██░░░░░███░░██░██░█░░░█░████░░░░█░░█░░░░░░░░░░░
░░░░░░░░░░░██░████░░██░█░░███░█░░█░████░░█░░███░░███░█░█░░░██░░███░░█░░██░██░█░░░█░░██████░░░░░░░░░░
░░░░░░░░░░██░░█░░░███░░████░░░████░█░░░██████░░███░░░█░██░██░███░░██████░░█░░██░█████░░░░░█░░░░░░░░░
```

Usage
-----

```bash
rule 30
```

use `--help` flag to learn about more options.

Ignorify
========
This is a personal tool that I built to generate gitignore files. It uses snippets 
based on the [GitHub ignore snippets](https://github.com/lynnjm7/gitignore).
The snippets, along with configuration files, are stored on the users local machine 
in `~/.ignorify/`. 

This project served two purposes: (1) it let me build a utility that I've wanted 
for a while and (2) it gave me a toy to play around with learning the Rust programming
language.

# Installation 
To install ignorify you will need to have Rust installed. For more information on
installing Rust see [the Rust installation page](https://www.rust-lang.org/en-US/install.html).

After you have installed Rust (which should include the `cargo` utility), you can 
install ignorify with the following command:

```bash
cargo install --git https://github.com/lynnjm7/ignorify
```

To verify that ignorify has been properly installed you should be able to run 

```bash
ignorify -V
```

which should show the ignorify version information.

To complete the installation, run the setup command that is built into the tool.
This will run the initiation steps to finish installing ignorify. See the 
[Setup](##setup) section of this document for more information.

# Usage
There are several different ways to use this app. For some instructions use

```bash
ignorify --help
```

## Setup
To setup this tool, simply run:

```bash
ignorify --init
```

This will clone the snippets directory down to the proper place in your home directory
and perform any other necessary setup tasks. After this init operation has been performed
you should be good to create ignore files as described below.

## Listing available options
To list the available snippets, use: 

```bash
ignorify --list
```

It is relatively trivial to be able to `grep` through this list to search 
for a particular snippet. For instance, 

```bash
ignorify --list | grep c++
```

## Generating gitignore files
To generate a gitignore file, simply type out the options that you would 
like to include in the ignore file: 

```bash
ignorify c++ windows linux macos vim emacs 
```

This will output that generated ignore file to stdout. To create the file 
use Unix command line redirection. I.e. 

```bash
ignorify c++ windows linux macos vim emacs > .gitignore
```

You can also use this to append new snippets to an existing ignore file: 

```bash
ignorify rust  >> .gitignore 
```

# Disclaimer
I have tested and use this tool on Linux. I haven't tested it on any other operating 
system. However, I see no reason why it shouldn't work and porting it should be relatively
straightforward for the sufficiently tenacious code monkey... But I do not currently have any plans 
on porting this tool to any other operating system.

# License
This project uses the MIT license. Please refer to LICENSE for more details. 


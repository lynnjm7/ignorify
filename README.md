Ignorify
========
This project is a personal tool that I built to generate gitignore files. It 
uses the snippets that are provided by GitHub. The snippets that are used to 
generate the actual gitignore files are stored in `~/.ignorify/snippets`. 


# Usage
There are several different ways to use this app. For some instructions use 
```
ignorify --help
```

To list the available snippets, use: 
```
ignorify --list
```

It is relatively trivial to be able to `grep` through this list to search 
for a particular snippet. For instance, 
```
ignorify --list | grep c++
```

To generate a gitignore file, simply type out the options that you would 
like to include in the ignore file: 
```
ignorify c++ windows linux macos vim emacs 
```

This will output that generated ignore file to stdout. To create the file 
use Unix command line redirection. I.e. 
```
ignorify c++ windows linux macos vim emacs > .gitignore
```

You can also use this to append new snippets to an existing ignore file: 
```
ignorify rust  >> .gitignore 
```

# License
This project uses the MIT license. Please refer to LICENSE for more details. 


# Bootstrap file
The bootstrap file contains the files and dependencies needed to run an elm program.
The file is unpacked into to the work dir by the code-runner.

###  Steps to make bootstrap.tar.gz
```
* elm init
* elm install #dependency
* tar -czf bootstrap.tar.gz elm.json .elm
```

# Bootstrap file
The bootstrap file contains dependencies needed to run a clojure program.
The dependencies are normally downloaded the first time you run a clojure program,
but that does not work inside a container without network access.
The file is unpacked into to the work dir by the code-runner.

###  Steps to make bootstrap.tar.gz
```
* docker run -u glot --rm -t -i glot/clojure:latest /bin/bash
* cd /home/glot
* rm -rf .m2
* echo '(println 42)' | clj

# Outside container: 
* docker cp -a '<containerid>:/home/glot/.m2' - | gzip > bootstrap.tar.gz
```

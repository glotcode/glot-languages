# Deps

###  Steps to make deps.tar
```
* docker run -u glot --rm -t -i glot/clojure:latest /bin/bash
* cd /home/glot
* echo '(println 42)' | clj

# Outside container: 
* docker cp -a '<containerid>:/home/glot/.m2' - > deps.tar
```

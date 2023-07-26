# RUN

To update dependency run :

```
REPIN=1 bazel run @unpinned_maven//:pin
```

this updates the deps for java binary

Then We can generate BUILD Files

```
bazel run //:gazelle
```

Once Build files are genereated we can build the jar files

```
bazel build //pkg:binaryname
```

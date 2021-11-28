# rust-html-parser-benchmark

A collection of benchmarks measuring raw performance (no interaction with the final tree) of HTML parsers written in Rust.

Note: some HTML parsers in this do not follow the full HTML specification.

## Running this test
`cargo bench` uses all of the HTML files in `data/` and passes them to each HTML parser. You can write the result to a file and pipe it into grep to filter out the irrelevant lines: `cargo bench > out.txt; cat out.txt | grep -A1 -B1 -- time:` (should give you the same output as below)

## Results
The benchmarks were run on GitHub codespaces. 
Note: some of these tests are not necessarily good examples of real world data, but can show performance or quirks of parsers on smaller HTML documents.


### wikipedia.html
```
html5ever-wikipedia.html/html5ever-wikipedia.html
  time:   [5.7573 ms 5.7645 ms 5.7717 ms]
  thrpt:  [54.192 MiB/s 54.260 MiB/s 54.327 MiB/s]
  
tl-wikipedia.html/tl-wikipedia.html
  time:   [705.44 us 706.45 us 707.47 us]
  thrpt:  [442.11 MiB/s 442.75 MiB/s 443.38 MiB/s]
  
htmlparser-wikipedia.html/htmlparser-wikipedia.html
  time:   [18.131 ms 18.155 ms 18.179 ms]
  thrpt:  [17.206 MiB/s 17.228 MiB/s 17.251 MiB/s]
  
rphtml-wikipedia.html/rphtml-wikipedia.html
  time:   [6.0143 ms 6.0223 ms 6.0305 ms]
  thrpt:  [51.867 MiB/s 51.937 MiB/s 52.006 MiB/s]
  
rusthtml-wikipedia.html/rusthtml-wikipedia.html
  time:   [3.3389 ms 3.3433 ms 3.3477 ms]
  thrpt:  [93.433 MiB/s 93.556 MiB/s 93.676 MiB/s]
  
htmlstream-wikipedia.html/htmlstream-wikipedia.html
  time:   [2.0316 ms 2.0344 ms 2.0372 ms]
  thrpt:  [153.54 MiB/s 153.75 MiB/s 153.96 MiB/s]
```

### medium-fragment.html
```
html5ever-medium-fragment.html/html5ever-medium-fragment.html
  time:   [51.734 us 51.816 us 51.899 us]
  thrpt:  [83.112 MiB/s 83.246 MiB/s 83.379 MiB/s]
  
tl-medium-fragment.html/tl-medium-fragment.html
  time:   [9.0587 us 9.0719 us 9.0855 us]
  thrpt:  [474.76 MiB/s 475.48 MiB/s 476.17 MiB/s]
  
htmlparser-medium-fragment.html/htmlparser-medium-fragment.html
  time:   [355.96 us 356.64 us 357.33 us]
  thrpt:  [12.072 MiB/s 12.095 MiB/s 12.118 MiB/s]
  
rphtml-medium-fragment.html/rphtml-medium-fragment.html
  time:   [89.472 us 89.682 us 89.952 us]
  thrpt:  [47.953 MiB/s 48.097 MiB/s 48.210 MiB/s]
  
rusthtml-medium-fragment.html/rusthtml-medium-fragment.html
  time:   [42.113 us 42.183 us 42.254 us]
  thrpt:  [102.08 MiB/s 102.26 MiB/s 102.43 MiB/s]
  
htmlstream-medium-fragment.html/htmlstream-medium-fragment.html
  time:   [28.172 us 28.260 us 28.373 us]
  thrpt:  [152.03 MiB/s 152.64 MiB/s 153.11 MiB/s]
```

### lipsum.html
```
html5ever-lipsum.html/html5ever-lipsum.html
  time:   [27.565 us 27.579 us 27.593 us]
  thrpt:  [437.73 MiB/s 437.95 MiB/s 438.17 MiB/s]
  
tl-lipsum.html/tl-lipsum.html
  time:   [5.8804 us 5.8882 us 5.8962 us]
  thrpt:  [2.0005 GiB/s 2.0032 GiB/s 2.0058 GiB/s]
  
htmlparser-lipsum.html/htmlparser-lipsum.html
  time:   [788.46 us 789.96 us 791.48 us]
  thrpt:  [15.260 MiB/s 15.290 MiB/s 15.319 MiB/s]
  
rphtml-lipsum.html/rphtml-lipsum.html
  time:   [49.346 us 49.425 us 49.505 us]
  thrpt:  [243.98 MiB/s 244.38 MiB/s 244.77 MiB/s]
  
rusthtml-lipsum.html/rusthtml-lipsum.html
  time:   [42.792 us 42.881 us 42.970 us]
  thrpt:  [281.09 MiB/s 281.67 MiB/s 282.25 MiB/s]
  
htmlstream-lipsum.html/htmlstream-lipsum.html
  time:   [15.926 us 15.954 us 15.982 us]
  thrpt:  [755.76 MiB/s 757.09 MiB/s 758.40 MiB/s]
  
```
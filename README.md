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
  time:   [5.7817 ms 5.7900 ms 5.7985 ms]
  thrpt:  [53.942 MiB/s 54.021 MiB/s 54.098 MiB/s]
  
tl-wikipedia.html/tl-wikipedia.html                                                                              
  time:   [627.03 us 628.23 us 629.48 us]
  thrpt:  [496.88 MiB/s 497.87 MiB/s 498.83 MiB/s]
  
htmlparser-wikipedia.html/htmlparser-wikipedia.html                                                                              
  time:   [17.738 ms 17.764 ms 17.790 ms]
  thrpt:  [17.582 MiB/s 17.608 MiB/s 17.634 MiB/s]
  
rphtml-wikipedia.html/rphtml-wikipedia.html                                                                              
  time:   [6.0053 ms 6.0154 ms 6.0256 ms]
  thrpt:  [51.909 MiB/s 51.997 MiB/s 52.084 MiB/s]
  
rusthtml-wikipedia.html/rusthtml-wikipedia.html                                                                              
  time:   [3.3830 ms 3.3881 ms 3.3933 ms]
  thrpt:  [92.175 MiB/s 92.317 MiB/s 92.455 MiB/s]
  
htmlstream-wikipedia.html/htmlstream-wikipedia.html                                                                              
  time:   [2.2752 ms 2.2786 ms 2.2822 ms]
  thrpt:  [137.05 MiB/s 137.27 MiB/s 137.48 MiB/s]
```

### medium-fragment.html
```
html5ever-medium-fragment.html/html5ever-medium-fragment.html                                                                               
  time:   [51.941 us 52.040 us 52.141 us]
  thrpt:  [82.727 MiB/s 82.888 MiB/s 83.046 MiB/s]

tl-medium-fragment.html/tl-medium-fragment.html                                                                               
  time:   [7.9936 us 8.0189 us 8.0470 us]
  thrpt:  [536.03 MiB/s 537.91 MiB/s 539.61 MiB/s]
  
htmlparser-medium-fragment.html/htmlparser-medium-fragment.html                                                                              
  time:   [360.03 us 360.72 us 361.42 us]
  thrpt:  [11.935 MiB/s 11.958 MiB/s 11.981 MiB/s]
  
rphtml-medium-fragment.html/rphtml-medium-fragment.html                                                                              
  time:   [90.184 us 90.354 us 90.535 us]
  thrpt:  [47.644 MiB/s 47.739 MiB/s 47.829 MiB/s]
  
rusthtml-medium-fragment.html/rusthtml-medium-fragment.html                                                                               
  time:   [42.938 us 43.021 us 43.105 us]
  thrpt:  [100.07 MiB/s 100.26 MiB/s 100.46 MiB/s]
  
htmlstream-medium-fragment.html/htmlstream-medium-fragment.html                                                                               
  time:   [31.329 us 31.393 us 31.460 us]
  thrpt:  [137.11 MiB/s 137.40 MiB/s 137.68 MiB/s]
```

### lipsum.html
```
html5ever-lipsum.html/html5ever-lipsum.html                                                                               
  time:   [24.222 us 24.255 us 24.288 us]
  thrpt:  [497.30 MiB/s 497.98 MiB/s 498.65 MiB/s]
  
tl-lipsum.html/tl-lipsum.html                                                                               
  time:   [817.68 ns 818.86 ns 820.06 ns]
  thrpt:  [14.383 GiB/s 14.404 GiB/s 14.425 GiB/s]

htmlparser-lipsum.html/htmlparser-lipsum.html                                                                              
  time:   [790.50 us 792.06 us 793.64 us]
  thrpt:  [15.219 MiB/s 15.249 MiB/s 15.279 MiB/s]

rphtml-lipsum.html/rphtml-lipsum.html                                                                               
  time:   [51.358 us 51.477 us 51.614 us]
  thrpt:  [234.01 MiB/s 234.64 MiB/s 235.18 MiB/s]
  
rusthtml-lipsum.html/rusthtml-lipsum.html                                                                               
  time:   [49.879 us 49.957 us 50.037 us]
  thrpt:  [241.39 MiB/s 241.77 MiB/s 242.15 MiB/s]
  
htmlstream-lipsum.html/htmlstream-lipsum.html                                                                               
  time:   [24.062 us 24.138 us 24.222 us]
  thrpt:  [498.65 MiB/s 500.38 MiB/s 501.96 MiB/s]  
```
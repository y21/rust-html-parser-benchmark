# rust-html-parser-benchmark

A collection of benchmarks measuring raw performance (no interaction with the final tree) of HTML parsers written in Rust.

**Note:** some HTML parsers in this do not follow the full HTML specification.

## Running this test
`cargo bench` uses all of the HTML files in `data/` and passes them to each HTML parser. You can write the result to a file and pipe it into grep to filter out the irrelevant lines: `cargo bench > out.txt; cat out.txt | grep -A1 -B1 -- time:` (should give you the same output as below)

## Results
Note: some of these tests are not necessarily good examples of real world data.

### wikipedia.html
```
html5ever-wikipedia.html/html5ever-wikipedia.html
  time:   [6.2213 ms 6.2233 ms 6.2255 ms]
  thrpt:  [50.242 MiB/s 50.259 MiB/s 50.276 MiB/s]

tl-wikipedia.html/tl-wikipedia.html
  time:   [629.01 us 629.78 us 630.59 us]
  thrpt:  [496.01 MiB/s 496.65 MiB/s 497.25 MiB/s]

htmlstream-wikipedia.html/htmlstream-wikipedia.html
  time:   [1.7992 ms 1.8008 ms 1.8025 ms]
  thrpt:  [173.52 MiB/s 173.69 MiB/s 173.84 MiB/s]

lol_html-wikipedia.html/lol_html-wikipedia.html
  time:   [788.51 us 788.91 us 789.36 us]
  thrpt:  [396.25 MiB/s 396.47 MiB/s 396.67 MiB/s]
```

### medium-fragment.html
```
html5ever-medium-fragment.html/html5ever-medium-fragment.html
  time:   [60.170 us 60.182 us 60.195 us]
  thrpt:  [71.658 MiB/s 71.673 MiB/s 71.689 MiB/s]

tl-medium-fragment.html/tl-medium-fragment.html
  time:   [7.9875 us 7.9999 us 8.0129 us]
  thrpt:  [538.32 MiB/s 539.19 MiB/s 540.03 MiB/s]

htmlstream-medium-fragment.html/htmlstream-medium-fragment.html
  time:   [23.131 us 23.148 us 23.166 us]
  thrpt:  [186.20 MiB/s 186.35 MiB/s 186.48 MiB/s]

lol_html-medium-fragment.html/lol_html-medium-fragment.html
  time:   [11.508 us 11.519 us 11.532 us]
  thrpt:  [374.03 MiB/s 374.45 MiB/s 374.84 MiB/s]
```

### lipsum.html
```
html5ever-lipsum.html/html5ever-lipsum.html
  time:   [23.506 us 23.532 us 23.560 us]
  thrpt:  [512.66 MiB/s 513.26 MiB/s 513.83 MiB/s]

tl-lipsum.html/tl-lipsum.html
  time:   [530.83 ns 531.60 ns 532.42 ns]
  thrpt:  [22.154 GiB/s 22.188 GiB/s 22.220 GiB/s]

htmlstream-lipsum.html/htmlstream-lipsum.html
  time:   [17.059 us 17.077 us 17.096 us]
  thrpt:  [706.49 MiB/s 707.28 MiB/s 708.01 MiB/s]

lol_html-lipsum.html/lol_html-lipsum.html
  time:   [5.9172 us 5.9258 us 5.9349 us]
  thrpt:  [1.9874 GiB/s 1.9905 GiB/s 1.9934 GiB/s]
```

// For the most part, this file was directly taken from the html5ever repository:
// https://github.com/servo/html5ever/blob/a40ae732e5f63abdfebee1645ca0343137148344/html5ever/benches/html5ever.rs

use std::fs::File;

use criterion::{black_box, Criterion, Throughput};
use html5ever::{
    tendril::{ByteTendril, ReadExt, SliceExt, StrTendril},
    tokenizer::{BufferQueue, TokenSink, TokenSinkResult, Tokenizer},
};

struct Sink;

impl TokenSink for Sink {
    type Handle = ();

    fn process_token(
        &mut self,
        token: html5ever::tokenizer::Token,
        line_number: u64,
    ) -> html5ever::tokenizer::TokenSinkResult<Self::Handle> {
        black_box(token);
        TokenSinkResult::Continue
    }
}

fn html5ever(c: &mut Criterion, path: &str) {
    let mut file = File::open(&path).ok().expect("can't open file");

    // Read the file and treat it as an infinitely repeating sequence of characters.
    let mut file_input = ByteTendril::new();
    file.read_to_tendril(&mut file_input)
        .ok()
        .expect("can't read file");
    let file_input: StrTendril = file_input.try_reinterpret().unwrap();
    let size = file_input.len();
    let mut stream = file_input.chars().cycle();

    // Break the input into chunks of 1024 chars (= a few kB).
    // This simulates reading from the network.
    let mut input = vec![];
    let mut total = 0usize;
    while total < size {
        // The by_ref() call is important, otherwise we get wrong results!
        // See rust-lang/rust#18045.
        let sz = std::cmp::min(1024, size - total);
        input.push(stream.by_ref().take(sz).collect::<String>().to_tendril());
        total += sz;
    }

    let name = format!("html5ever-{}", path.split("/").last().unwrap());

    let mut group = c.benchmark_group(&name);
    group.sample_size(10);
    group.throughput(Throughput::Bytes(total.try_into().unwrap()));
    group.significance_level(0.1).sample_size(20000);
    group.bench_function(&name, |b| {
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(Sink, Default::default());
            let mut buffer = BufferQueue::new();

            for buf in input.clone().into_iter() {
                buffer.push_back(buf);
                let _ = tokenizer.feed(&mut buffer);
            }
            let _ = tokenizer.feed(&mut buffer);
            tokenizer.end();
        })
    });
}

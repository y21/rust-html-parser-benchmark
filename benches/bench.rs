use criterion::*;
use lol_html::HtmlRewriter;
use lol_html::Settings;

include!("html5ever.rs");

fn register_benchmark<F>(cr: &mut Criterion, input: &str, name: &str, mut code: F)
where
    F: FnMut(),
{
    let input = &input;

    let mut group = cr.benchmark_group(name);
    group.throughput(Throughput::Bytes(input.as_bytes().len() as u64));
    group.significance_level(0.1).sample_size(10000);
    group.bench_function(name, |b| {
        b.iter(|| {
            code();
        });
    });
    group.finish();
}

pub fn criterion_benchmark(cr: &mut Criterion) {
    let files = std::fs::read_dir("data").unwrap();

    for file in files {
        let file = file.unwrap().file_name();
        let file = file.to_str().unwrap();

        let path = format!("data/{}", file);

        // setup html5ever
        html5ever(cr, &path);

        let input = std::fs::read_to_string(&path).unwrap();

        // setup other parsers
        register_benchmark(cr, &input, &format!("tl-{}", file), || {
            let _ = tl::parse(&input, tl::ParserOptions::default());
        });

        register_benchmark(cr, &input, &format!("htmlstream-{}", file), || {
            let _ = htmlstream::tag_iter(&input).collect::<Vec<_>>();
        });

        register_benchmark(cr, &input, &format!("lol_html-{}", file), || {
            let mut v = Vec::new();
            let mut rewriter = HtmlRewriter::new(Settings::default(), |c: &[u8]| {
                v.extend_from_slice(c);
            });

            let _ = rewriter.write(input.as_bytes());
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// The `scoped_futures` function fails to compile for me unless I uncomment the feature flag.
// `test_scope_and_block` does compile, although I'm not clear on why.
// #![feature(async_closure)]

async fn scoped_futures() {
    let not_copy = String::from("hello world!");
    let not_copy_ref = &not_copy;
    let (mut stream, _) = unsafe {
        async_scoped::scope(|s| {
            for _ in 0..10 {
                let proc = async || {
                    assert_eq!(not_copy_ref, "hello world!");
                };
                s.spawn(proc());
            }
        })
    };
}

async fn test_scope_and_block() {
    let not_copy = String::from("hello world!");
    let not_copy_ref = &not_copy;

    let (_, vals) = async_scoped::scope_and_block(|s| {
        for _ in 0..10 {
            let proc = || async {
                assert_eq!(not_copy_ref, "hello world!");
            };
            s.spawn(proc());
        }
    });

    assert_eq!(vals.len(), 10);
}

#[async_std::main]
async fn main() {}

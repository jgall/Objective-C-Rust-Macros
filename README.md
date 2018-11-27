## Objective-c Rust macros

Why does this exist? Who knows... but it lets you do cool stuff like this:

        let my_class = register_class!(MyClass:NSObject with {
            (pub width: u32),
            (priv height: u32),
            (sel add:(i32)t1 with:(i32)t2 -> i32 with |obj, sel| {
                return t1+t2;
            }),
        });
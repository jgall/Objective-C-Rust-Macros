## Objective-c Rust macros

Why does this exist? Who knows... but it lets you do cool stuff like this:

        let my_class = register_class!(MyClass:NSObject with {
            (pub width: u32),
            (priv height: u32),
            (sel add:(i32)t1 with:(i32)t2 -> i32 with |obj, sel| {
                return t1+t2;
            }),
        });
        
        let obj = unsafe { msg_send![msg_send![my_class, alloc], init] };
        let x: i32 = unsafe { msg_send![*obj, add:5 with:6] };
        assert_eq!(x, 11);

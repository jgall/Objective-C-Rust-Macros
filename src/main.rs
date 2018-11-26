#[macro_use]
extern crate objc;

use objc::declare::ClassDecl;
use objc::rc::StrongPtr;
use objc::runtime::{Class, Object, Sel};
use objc::Encode;

fn main() {
    //test1();
    //println!("--------- END OF TEST 1 ----------");
    test2();
}

fn test2() {
    register_my_num();
}

macro_rules! add_pub_ivar {
    ($name:ident, $decl:expr, $type:ident) => {{
        $decl.add_ivar::<$type>(concat!("_", stringify!($name)));
        extern "C" fn getter(this: &Object, _cmd: Sel) -> $type {
            unsafe { *this.get_ivar(concat!("_", stringify!($name))) }
        }
        unsafe {
            $decl.add_method(sel!($type), getter as extern "C" fn(&Object, Sel) -> $type);
        }
        concat!("_", stringify!($name))
    }};
}

macro_rules! register_class {
    ($name:ident : $parent:ident with {
        $($field_name:ident : $ty_name:ident,)*
    }) => {{
        let superclass = class!($parent);
        let mut my_class = ClassDecl::new(stringify!($name), superclass).unwrap();
        $(
            add_pub_ivar!($field_name, my_class, $ty_name);
        )*
        my_class.register()
    }};
}

fn register_my_num() {
    let my_box_class = register_class!(MyBox:NSObject with {
        width: u32,
        height: u32,
    });
}

fn test1() {
    // Get a class
    let cls = class!(NSObject);
    println!("NSObject size: {}", cls.instance_size());

    // Inspect its ivars
    println!("NSObject ivars:");
    for ivar in cls.instance_variables().iter() {
        println!("{}", ivar.name());
    }

    // Allocate an instance
    let obj = unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, init];
        StrongPtr::new(obj)
    };
    println!("NSObject address: {:p}", obj);

    // Access an ivar of the object
    let isa: *const Class = unsafe { *(**obj).get_ivar("isa") };
    println!("NSObject isa: {:?}", isa);

    // Inspect a method of the class
    let hash_sel = sel!(hash);
    let hash_method = cls.instance_method(hash_sel).unwrap();
    let hash_return = hash_method.return_type();
    println!("-[NSObject hash] return type: {:?}", hash_return);
    assert!(hash_return == usize::encode());

    // Invoke a method on the object
    let hash: usize = unsafe { msg_send![*obj, hash] };
    println!("NSObject hash: {}", hash);
}

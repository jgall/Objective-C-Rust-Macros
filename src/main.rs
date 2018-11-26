#[macro_use]
extern crate objc;

use objc::declare::ClassDecl;
use objc::rc::StrongPtr;
use objc::runtime::{Class, Object, Sel};
use objc::Encode;

#[macro_use]
mod class;

fn main() {
    register_my_num();
}

fn register_my_num() {
    extern "C" fn obj_method(obj: &mut Object, sel: Sel, i1: i32, i2: i32) -> i32 {
        return 3;
    }
    let my_method: extern "C" fn(&mut Object, Sel, i32, i32) -> i32 = obj_method;

    let my_box_class = register_class!(MyBox:NSObject with {
        (pub width: u32),
        (priv height: u32),
        (sel getThing:withOtherThing: <- my_method),
    });
    print_class_stats(my_box_class)
}

fn print_class_stats(cls: &objc::runtime::Class) {
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

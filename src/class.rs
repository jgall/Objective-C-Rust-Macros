use objc::declare::ClassDecl;
use objc::rc::StrongPtr;
use objc::runtime::{Class, Object, Sel};
use objc::Encode;

#[macro_export]
macro_rules! add_pub_ivar {
    (pub, $name:ident, $decl:expr, $type:ident) => {{
        $decl.add_ivar::<$type>(concat!("_", stringify!($name)));
        extern "C" fn getter(this: &Object, _cmd: Sel) -> $type {
            unsafe { *this.get_ivar::<$type>(concat!("_", stringify!($name))) }
        }
        let getter_extern: extern "C" fn(&Object, Sel) -> $type = getter;
        unsafe {
            $decl.add_method(sel!($name), getter_extern);
        }
        extern "C" fn setter(this: &mut Object, _cmd: Sel, value: $type) {
            unsafe {
                this.set_ivar::<$type>(concat!("_", stringify!($name)), value);
            }
        }
        let extern_setter: extern "C" fn(&mut Object, Sel, $type) = setter;
        unsafe {
            $decl.add_method(
                sel_impl!(concat!(stringify!($name), ':', '\0')),
                extern_setter,
            );
        }
        concat!("_", stringify!($name))
    }};
    (priv, $name:ident, $decl:expr, $type:ident) => {{
        $decl.add_ivar::<$type>(concat!("_", stringify!($name)));
        concat!("_", stringify!($name))
    }};
}

macro_rules! process_field {
    ($class_dec:expr, (sel $($sel_name:ident :)* <- $fun_name:expr)) => {{
        unsafe {
            $class_dec.add_method(sel!($($sel_name :)*), $fun_name)
        }
    }};
    ($class_dec:expr, ($access:ident $field_name:ident : $ty_name:ident)) => {
        add_pub_ivar!($access, $field_name, $class_dec, $ty_name);
    };
}

#[macro_export]
macro_rules! register_class {
    ($name:ident : $parent:ident with {
        $($addition:tt,)*
    }) => {{
        let superclass = class!($parent);
        let mut my_class = ClassDecl::new(stringify!($name), superclass).unwrap();
        $(
            process_field!(my_class, $addition);
            //add_pub_ivar!($access, $field_name,$ty_name);
        )*
        my_class.register()
    }};
}

fn register_test() {
    extern "C" fn obj_method(obj: &mut Object, sel: Sel, i1: i32, i2: i32) -> i32 {
        return 3;
    }
    let protocol_class_method: extern "C" fn(&mut Object, Sel, i32, i32) -> i32 = obj_method;

    let my_box_class = register_class!(MyBox:NSObject with {
        (pub width: u32),
        (priv height: u32),
        (sel getThing:withOtherThing: <- protocol_class_method),
    });
}
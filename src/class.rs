use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};

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
    ($class_dec:expr, (sel $($sel_name:ident : ($sel_type:ident) $sel_local_name:ident)*
        -> $ret_type:ident with |$obj:ident, $sel:ident| $body:tt)) => {{
        extern "C" fn selector_fun($obj: &mut Object, $sel: Sel, $($sel_local_name:$sel_type),*) -> $ret_type $body
        let selector_fun_extern: extern "C" fn(&mut Object, Sel, $($sel_type),*) -> $ret_type = selector_fun;

        unsafe {
            $class_dec.add_method(sel!($($sel_name:)*), selector_fun_extern);
        }
    }};
    ($class_dec:expr, (static sel $($sel_name:ident : ($sel_type:ident) $sel_local_name:ident)*
        -> $ret_type:ident with |$cls:ident, $sel:ident| $body:tt)) => {{
        extern "C" fn selector_fun($cls: &Class, $sel: Sel, $($sel_local_name:$sel_type),*) -> $ret_type $body
        let selector_fun_extern: extern "C" fn(&Class, Sel, $($sel_type),*) -> $ret_type = selector_fun;

        unsafe {
            $class_dec.add_class_method(sel!($($sel_name:)*), selector_fun_extern);
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

#[cfg(test)]
mod test {
    use objc::rc::StrongPtr;
    #[macro_use]
    use super::*;

    #[test]
    fn register_test() {
        extern "C" fn obj_method(obj: &mut Object, sel: Sel, i1: i32, i2: i32) -> i32 {
            return 11;
        }
        let my_method: extern "C" fn(&mut Object, Sel, i32, i32) -> i32 = obj_method;

        let my_box_class = register_class!(MyBox:NSObject with {
            (pub width: u32),
            (priv height: u32),
            (sel getThing:withOtherThing: <- my_method),
            (sel add:(i32)t1 with:(i32)t2 -> i32 with |obj, sel| {
                return t1+t2;
            }),
            (static sel mul:(i32)t1 with:(i32)t2 -> i32 with |cls, sel| {
                return t1*t2;
            }),
        });
        let obj = unsafe {
            let obj: *mut Object = msg_send![my_box_class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            StrongPtr::new(obj)
        };
        let x: i32 = unsafe { msg_send![*obj, add:5i32 with:6i32] };

        assert_eq!(x, 11);

        let y: i32 = unsafe { msg_send![my_box_class, mul:5 with:6] };
        assert_eq!(y, 30);
    }
}

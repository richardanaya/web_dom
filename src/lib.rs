#![no_std]
use js_ffi::*;

struct Dom {
    fn_query_selector: JSInvoker,
    fn_add_event_listener: JSInvoker,
}

impl Default for Dom {
    fn default() -> Self {
        Dom {
            fn_query_selector: js!(Element.prototype.querySelector),
            fn_add_event_listener: js!(Node.prototype.addEventListener),
        }
    }
}

impl Dom {
    fn query_selector(&self, el: &JSObject, selector: &str) -> JSObject {
        JSObject(self.fn_query_selector.call_1(el, selector))
    }

    fn add_event_listener(&self, el: &JSObject, event_type: &str, callback: JSFunction) {
        self.fn_add_event_listener
            .call_2(el, event_type, callback);
    }
}

pub fn query_selector(el: &JSObject, selector: &str) -> JSObject {
    globals::get::<Dom>().query_selector(el, selector)
}

pub fn add_event_listener(el: &JSObject, event_type: &str, callback: JSFunction) {
    globals::get::<Dom>().add_event_listener(el, event_type, callback);
}

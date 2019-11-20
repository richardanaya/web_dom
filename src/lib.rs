#![no_std]
use js_ffi::*;

pub struct Console {
    fn_log:JSInvoker,
    fn_clear:JSInvoker,
    fn_error:JSInvoker,
    fn_warning:JSInvoker,
    fn_time:JSInvoker,
    fn_time_end:JSInvoker,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            fn_log:js!(console.log),
            fn_clear:js!(console.clear),
            fn_error:js!(console.error),
            fn_warning:js!(console.warn),
            fn_time:js!(console.time),
            fn_time_end:js!(console.timeEnd),
        }
    }
}

impl Console {
    pub fn clear(&self){
        self.fn_clear.invoke_0();
    }

    pub fn log(&self,msg:&str){
        self.fn_log.invoke_1(msg);
    }

    pub fn warning(&self,msg:&str){
        self.fn_warning.invoke_1(msg);
    }

    pub fn error(&self,msg:&str){
        self.fn_error.invoke_1(msg);
    }

    pub fn time(&self,label:Option<&str>){
        if label.is_none() {
            self.fn_time.invoke_0();
        } else {
            self.fn_time.invoke_1(label.unwrap());
        }
    }

    pub fn time_end(&self,label:Option<&str>){
        if label.is_none() {
            self.fn_time_end.invoke_0();
        } else {
            self.fn_time_end.invoke_1(label.unwrap());
        }
    }
}
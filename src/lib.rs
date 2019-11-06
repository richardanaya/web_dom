#![no_std]
use js_ffi::*;

pub struct Console {
    fn_log:JSValue,
    fn_clear:JSValue,
    fn_error:JSValue,
    fn_warning:JSValue,
    fn_time:JSValue,
    fn_time_end:JSValue,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            fn_log:register("console.log"),
            fn_clear:register("console.clear"),
            fn_error:register("console.error"),
            fn_warning:register("console.warn"),
            fn_time:register("console.time"),
            fn_time_end:register("console.timeEnd"),
        }
    }
}

impl Console {
    pub fn clear(&self){
        call_0(UNDEFINED,self.fn_clear);
    }

    pub fn log(&self,msg:&str){
        call_1(UNDEFINED,self.fn_log,TYPE_STRING,to_js_string(msg));
    }

    pub fn warning(&self,msg:&str){
        call_1(UNDEFINED,self.fn_warning,TYPE_STRING,to_js_string(msg));
    }

    pub fn error(&self,msg:&str){
        call_1(UNDEFINED,self.fn_error,TYPE_STRING,to_js_string(msg));
    }

    pub fn time(&self,label:Option<&str>){
        if label.is_none() {
            call_0(UNDEFINED,self.fn_time);
        } else {
            call_1(UNDEFINED,self.fn_time,TYPE_STRING,to_js_string(label.unwrap()));
        }
    }

    pub fn time_end(&self,label:Option<&str>){
        if label.is_none() {
            call_0(UNDEFINED,self.fn_time_end);
        } else {
            call_1(UNDEFINED,self.fn_time_end,TYPE_STRING,to_js_string(label.unwrap()));
        }
    }
}
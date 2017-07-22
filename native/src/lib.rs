#[macro_use]
extern crate neon;
extern crate gears;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsBoolean};
use neon::js::error::{JsError, Kind};

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}



register_module!(m, {
    m.export("hello", hello)
});

use boa_engine::{object::builtins::JsPromise, Context, JsResult, JsValue, NativeFunction, Source};
use std::future::Future;

pub fn test_js_func(
    _this: &JsValue,
    args: &[JsValue],
    _context: &mut Context<'_>,
) -> impl Future<Output = JsResult<JsValue>> {
    let arg = args.get(0).cloned();
    async move {
        std::future::ready(()).await;
        if let Some(arg) = &arg {
            if arg.is_string() {
                if let Some(arg) = arg.as_string() {
                    println!("the var is: {}", arg.to_std_string_escaped());
                    return Ok(JsValue::from(arg.clone()));
                }
            }
        }
        drop(arg);
        Ok(JsValue::null())
    }
}

#[tokio::main]
async fn main() {
    let ctx = &mut Context::builder().build().unwrap();

    ctx.register_global_callable("testCall", 0, NativeFunction::from_async_fn(test_js_func))
        .unwrap();

    let call = "testCall(\"testing argument\")";

    let result = ctx.eval(Source::from_bytes(&call)).unwrap();
    println!("{:?}", result);
    if result.is_string() {
        if let Some(result) = result.as_string() {
            println!(
                "the original call sees as result: {}",
                result.to_std_string_escaped()
            );
        }
    }

    let (promise, _resolvers) = JsPromise::new_pending(ctx);
    let promise_future = promise.into_js_future(ctx).unwrap();
    ctx.run_jobs_async().await;

    let result_js = promise_future.await.unwrap();
    println!("{:?}", result_js);
}

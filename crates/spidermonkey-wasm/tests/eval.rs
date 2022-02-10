mod eval {
    use spidermonkey_wasm::{jsapi, rooted::Rooted, runtime::Runtime};
    use std::ptr;

    #[test]
    fn eval() {
        let runtime = Runtime::default();
        let global_class = jsapi::MakeDefaultGlobalClass();
        let context = runtime.cx();

        unsafe {
            let realm_opts = jsapi::MakeDefaultRealmOptions();
            let mut default_global_root = jsapi::jsgc::Rooted::default();
            let global_object = Rooted::new(
                context,
                &mut default_global_root,
                jsapi::JS_NewGlobalObject(
                    runtime.cx(),
                    &*global_class,
                    ptr::null_mut(),
                    jsapi::OnNewGlobalHookOption::FireOnNewGlobalHook,
                    &realm_opts,
                ),
            );

            let global_object_handle = global_object.handle();
            let _ar = jsapi::jsrealm::JSAutoRealm::new(context, global_object_handle.get());
            let owning_compile_options = jsapi::MakeOwningCompileOptions(
                context,
                &jsapi::CompileOptionsParams {
                    force_full_parse: false,
                    lineno: 1,
                    file: "eval.js".into(),
                },
            );

            let mut unrooted_return_value = jsapi::jsgc::Rooted::default();
            let mut return_value =
                Rooted::new(context, &mut unrooted_return_value, jsapi::UndefinedValue());
            let mut return_value_handle = return_value.mut_handle();

            let script = "41 + 1";
            let mut source = jsapi::MakeUtf8UnitSourceText();
            assert!(jsapi::InitUtf8UnitSourceText(
                context,
                source.pin_mut(),
                &script,
                script.len(),
                jsapi::SourceOwnership::Borrowed
            ));

            jsapi::Utf8SourceEvaluate(
                context,
                &owning_compile_options,
                source.pin_mut(),
                return_value_handle.into_raw(),
            );

            let result = return_value.get().toInt32();
            assert_eq!(result, 42);
        }
    }
}

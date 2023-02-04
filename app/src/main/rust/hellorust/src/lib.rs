use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_com_kokoadev_ndkwithrust_MainActivity_stringFromJNI(
   env: JNIEnv,
   _this: JObject,
) -> jstring {
   let hello = "Hello from Rust";

   // env.new_stringはJavaのStringを生成する
   // Options.expectは値がSomeの場合は値を返し、そうでない場合はpanic
   // into_innerは生のポインターを返す
   env.new_string(hello)
       .expect("Couldn't create Java string!")
       .into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_kokoadev_ndkwithrust_MainActivity_helloNameFromJNI(
   env: JNIEnv,
   _this: JObject,
   input: JString
) -> jstring {
   // intoはFromの逆の働き
   // この場合はJavaStrをStringに変換してくれている
   let input: String = env.get_string(input).expect("Could not get java string").into();

   let output = env.new_string(format!("Hello {}!", input))
       .expect("Could not create java string");
   output.into_inner()
}

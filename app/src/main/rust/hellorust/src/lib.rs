use jni::JNIEnv;
use jni::objects::{JObject, JString, ReleaseMode};
use jni::sys::{jint, jintArray, jstring};

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
    input: JString,
) -> jstring {
    // intoはFromの逆の働き
    // この場合はJavaStrをStringに変換してくれている
    let input: String = env.get_string(input).expect("Could not get java string").into();

    let output = env.new_string(format!("Hello {}!", input))
        .expect("Could not create java string");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_kokoadev_ndkwithrust_MainActivity_addNumberFromJNI(
    _env: JNIEnv,
    _this: JObject,
    input1: jint,
    input2: jint,
) -> jint {
    // Rustのi32に変換する
    let num1 = input1 as i32;
    let num2 = input2 as i32;
    let result = num1 + num2;
    result as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_kokoadev_ndkwithrust_MainActivity_sortIntArrayFromJNI(
    env: JNIEnv,
    _this: JObject,
    list: jintArray,
) -> jintArray {

    //get_int_array_elementsでintArrayを取る
    let auto_array = env.get_int_array_elements(list, ReleaseMode::CopyBack).unwrap();
    let len = auto_array.size().unwrap() as usize;
    let slice = unsafe { std::slice::from_raw_parts_mut(auto_array.as_ptr(), len) };
    let mut vec = Vec::from(slice);

    vec.sort();
    vec.sort_by_bubble_algorithm();
    // bubble_sort(&mut vec);
    quick_sort(&mut vec);
    let int_array = env.new_int_array(vec.len() as i32).unwrap();
    env.set_int_array_region(int_array, 0, &vec).unwrap();

    int_array
}


fn bubble_sort(arr: &mut [jint]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
}

fn quick_sort(arr: &mut [jint]) {
    if arr.len() > 1 {
        let pivot = partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[(pivot + 1)..]);
    }
}

fn partition(arr: &mut [jint]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    let mut i = 0;
    let j = len - 1;
    arr.swap(pivot_index, j);
    for k in 0..(len - 1) {
        if arr[k] < arr[j] {
            arr.swap(k, i);
            // i はpivotより小さい数値のカウント
            i += 1;
        }
    }
    // 最後のpivotを元に戻すことで左に小さい値、右に大きい値が並ぶ
    arr.swap(i, j);
    i
}

trait MyBubbleSort {
    fn sort_by_bubble_algorithm(&mut self);
}

impl MyBubbleSort for [jint] {
    fn sort_by_bubble_algorithm(&mut self) {
        bubble_sort(self)
    }
}

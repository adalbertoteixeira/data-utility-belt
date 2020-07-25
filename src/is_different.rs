use serde_json::{json, Map, Value};
use std::convert::TryFrom;
use std::io::{self, Write};

pub fn is_different(
    object: &Value,
    object_to_compare: &Value,
    key: &String,
    object_differences: &mut Value,
    parent_key: &Vec<String>,
    parent_pointer: &mut str,
    debug: bool,
) {
    let mut pointer = &parent_pointer.to_owned();
    if debug {
        println!(
            "IS DIFFERENT INITIAL: \n\tPOINTER: {:?}\n\tOBJECT: {:?}\n",
            pointer, object
        );
    }
    let object_original_pointer = &object.pointer(&pointer);
    let object_to_compare_pointer = &object_to_compare.pointer(&pointer);
    if debug {
        println!(
            "\tOBJECTS: \n\t\toriginal:{:?}\n \t\tto compare:{:?}\n",
            object_original_pointer, object_to_compare_pointer
        );
    }
    let key_original = &object.get(key);
    let key_to_compare = &object_to_compare.get(key);

    if key_to_compare.is_none() {
        *object_differences.pointer_mut("/x").unwrap() = json!([object[key].clone(), null]);
        return;
    }

    if object_original_pointer.unwrap().is_array() {
        if debug {
            println!("\t\t\tIS ARRAY!");
        }
        let object_as_array = object_original_pointer.unwrap().as_array().unwrap();
        if object_as_array.len() != object_to_compare_pointer.unwrap().as_array().unwrap().len() {
            // @TODO
            return;
        }

        if debug {
            println!("\t\t\tARRAY IS DIFFERENT");
        }
        for (i, e) in object_as_array.iter().enumerate() {
            if debug {
                println!(
                    "\t\t\tArray iteration:\n\t\t\t\ti: {:?}\n\t\t\t\t{:?}",
                    i, e
                );
            }
            let mut array_pointer = pointer.clone();
            array_pointer.push_str("/");
            array_pointer.push_str(&i.to_string());
            is_different(
                &object,
                &object_to_compare,
                &key.to_string(),
                object_differences,
                &parent_key,
                &mut array_pointer,
                debug,
            );
        }
    }

    if key_original.unwrap() != key_to_compare.unwrap() {
        if debug {
            println!(
                "\tComparing value\t\t
                Key(s): {:?}, {:?}\t\t
                Pointer: {:?}\t\t
                Object: {:?},\t\t
                Object to compare: {:?}\t\t
                Value(s){:?}, {:?}
                object_differences: {:?}
                ",
                parent_key,
                key,
                pointer,
                object,
                object_to_compare,
                key_original.unwrap(),
                key_to_compare.unwrap(),
                object_differences
            );
            object_differences.pointer_mut(&pointer).map(|v| {
                println!("\tObject differences value: {:?}\n", *v);
            });
        }

        let pointer_collect: Vec<&str> = pointer.split("/").collect();
        let mut from_value: Value = serde_json::from_value(object_differences.clone()).unwrap();
        from_value["V"] = json!({});
        if debug {
            println!(
                "\n\tPointer collect: {:?}
                from value: {:?}",
                pointer_collect, from_value
            );
        }

        let mut tmp_pointer = "/".to_owned();
        let mut tmp_object_differences_pointer = &object_differences;

        if debug {
            println!("\tCurrent pointer before iteration: {:?}", tmp_pointer);
        }
        for (index, element) in pointer_collect.iter().enumerate() {
            let i = element;
            let previous_pointer = tmp_pointer.clone();
            if debug {
                println!(
                    "\tPointer collect iterate:
                    iter_pointer: {:?}
                    index: {:?} ",
                    i,
                    i.len()
                );
            }
            if i.len() == 0 {
                if debug {
                    println!("\tCurrent pointer inside loop: {:?}", tmp_pointer);
                    println!("\tIterator length is 0: {:?}", i);
                    println!(
                        "\tCurrent object_differences {:?}",
                        &object_differences.clone()
                    );
                }
            }
            // if i.len() > 0 {
            if debug {
                println!("\tCurrent pointer inside loop: {:?}", tmp_pointer);
                println!("\tIterator length is greater than 0: {:?}", i);
                println!(
                    "\tCurrent object_differences {:?}",
                    &object_differences.clone()
                );
            }
            if index > 1 {
                tmp_pointer.push_str("/");
            }
            tmp_pointer.push_str(&i);
            if debug {
                println!(
                    "\t\tCurrent pointer inside loop after pushing i: {:?} ",
                    tmp_pointer,
                );
            }
            //@ TODO:
            // 1 - Check pointer exists
            //
            let current_value_in_tmp_differences_pointer = object_differences.get(&tmp_pointer);
            // &object_differences.pointer(&tmp_pointer);
            if debug {
                println!(
                    "\n\n\t\tcurrent_value_in_tmp_differences_pointer: {:?}",
                    current_value_in_tmp_differences_pointer
                );
            }

            if current_value_in_tmp_differences_pointer.is_none() {
                if debug {
                    println!("\n\n\t\tprevious_pointer: {:?}", previous_pointer);
                    println!(
                        "\n\n\t\tcurrent_value_in_tmp_differences_pointer is NONE: {:?}",
                        current_value_in_tmp_differences_pointer
                    );
                }
                if index == 1 {
                    object_differences[&i] = json!({});
                    // *object_differences.pointer_mut(&tmp_pointer).unwrap() = {}.into();
                }
                if index > 1 {
                    *object_differences.pointer_mut(&previous_pointer).unwrap() = {}.into();
                    println!(
                        "Object_differences after inserting i with index {:?} :{:?}",
                        index, object_differences
                    );
                }
                // object_differences[&i] = json!({});
                // *object_differences
                //     .pointer_mut(&tmp_pointer.to_owned())
                //     .unwrap() = {}.into();
            }

            //@ TODO:
            // if not add it
            // object_differences[&i] = json!({});
            println!(
                "\tObject_differences after inserting i {:?}",
                object_differences
            );
            // *object_differences.pointer_mut(&"/test".to_owned()).unwrap() = {}.into();
            // }
        }
        // println!(
        // "END: {:?}, {:?}, {:?},\n {:?}\n {:?}",
        // pointer,
        // object_original_pointer,
        // object_to_compare_pointer,
        // object_differences,
        // pointer_collect
        // );

        if object_original_pointer.unwrap() != object_to_compare_pointer.unwrap() {
            // *object_differences.pointer_mut(&pointer).unwrap() = json!([
            //     object_original_pointer.unwrap(),
            //     object_to_compare_pointer.unwrap()
            // ]);
        }
    }
}

// #[test]
// pub fn should_equal_numbers() {
//     let mut object_differences = json!({});
//     let mut o: Map<String, Value> = Map::new();
//     let mut o_c: Map<String, Value> = Map::new();
//     let key = "text".to_string();
//     o.insert(key.clone(), json!(1));
//     o_c.insert(key.clone(), json!(1));
//     let parent_key: Vec<String> = Vec::new();
//     is_different(&o, &o_c, &key, &mut object_differences, &parent_key);
//     let result = json!({});
//     assert_eq!(object_differences, result);
// }

// #[test]
// pub fn should_equal_strings() {
//     let mut object_differences = json!({});
//     let mut o: Map<String, Value> = Map::new();
//     let mut o_c: Map<String, Value> = Map::new();
//     let key = "text".to_string();
//     o.insert(key.clone(), json!("string"));
//     o_c.insert(key.clone(), json!("string"));
//     let parent_key: Vec<String> = Vec::new();
//     is_different(&o, &o_c, &key, &mut object_differences, &parent_key);
//     let result = json!({});
//     assert_eq!(object_differences, result);
// }

// #[test]
// pub fn should_equal_sorted_arrays() {
//     let mut object_differences = json!({});
//     let mut o: Map<String, Value> = Map::new();
//     let mut o_c: Map<String, Value> = Map::new();
//     let key = "array".to_string();
//     o.insert(key.clone(), json!([1, 2, 3, 4]));
//     o_c.insert(key.clone(), json!([1, 2, 3, 4]));
//     let parent_key: Vec<String> = Vec::new();
//     is_different(&o, &o_c, &key, &mut object_differences, &parent_key);
//     let result = json!({});
//     assert_eq!(object_differences, result);
// }

// #[test]
// pub fn should_equal_unsorted_arrays() {
//     let mut object_differences = json!({});
//     let mut o: Map<String, Value> = Map::new();
//     let mut o_c: Map<String, Value> = Map::new();
//     let key = "array".to_string();
//     o.insert(key.clone(), json!([4, 3, 2, 1]));
//     o_c.insert(key.clone(), json!([1, 2, 3, 4]));
//     let parent_key: Vec<String> = Vec::new();
//     is_different(&o, &o_c, &key, &mut object_differences, &parent_key);
//     let result = json!({});
//     assert_eq!(object_differences, result);
// }

#[test]
pub fn should_error_array_with_different_values_of_same_type() {
    let mut object_differences = json!({});
    let mut o = json!({ "array":[4] });
    let mut o_c = json!({"array": [1] });
    let key = "array".to_string();
    // o.insert(key.clone(), json!([4]));
    // o_c.insert(key.clone(), json!([1]));
    let parent_key: Vec<String> = Vec::new();
    println!("{:?}", object_differences);
    let mut parent_pointer = "/array".to_owned();
    is_different(
        &o,
        &o_c,
        &key,
        &mut object_differences,
        &parent_key,
        &mut parent_pointer,
        true,
    );
    let mut result = json!({
        "array": [4, 1]
    });
    // result.insert("array".to_string(), json!([4, 1]));
    println!("{:?}, {:?}", &object_differences, result);
    assert_eq!(object_differences, result);
}

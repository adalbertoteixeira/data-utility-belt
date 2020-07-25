#[test]
pub fn is_array_different() {
    is_different();
    let mut object_differences = Map::new();
    let object = object_matches.value_of("object").unwrap();
    let parsed_object: Value = serde_json::from_str(object).unwrap();
    let o: Map<String, Value> = parsed_object.as_object().unwrap().clone();
    let object_compare = object_matches.value_of("object_to_compare").unwrap();
    let parsed_object_compare: Value = serde_json::from_str(object_compare).unwrap();
    let o_c: Map<String, Value> = parsed_object_compare.as_object().unwrap().clone();
    for key in o.keys() {
        // object_differences =
        let mut parent_key: Vec<String> = Vec::new();
        is_different(&o, &o_c, &key, &mut object_differences, &parent_key);
        // println!(
        //     "KEY: {:?}, {:?}  {:?}\n\n\n",
        //     key,
        //     o_c.get(key).is_none(),
        //     o_c.get(key).is_some(),
        // );

        // if is_diff {
        // object_differences
        // .insert(key.to_string(), json!([o[key].clone(), o_c[key].clone()]));
        // }
    }
    assert_eq!(add(2, 3), 5);
}

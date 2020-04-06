extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json::{json, Map, Result, Value};
use std::collections::HashMap;

fn main() {
    let matches = App::new("Data utility Belt")
                .version("0.1")
                .author("Adalberto Teixeira")
        .subcommand(
            SubCommand::with_name("array")
                .about("parses array elements")
                .version("0.1")
                .author("Adalberto Teixeira")
                .help("Perform operations on arrays. Array is always sorted, even when no other operations are performed.")
                .arg(
                    Arg::with_name("unique")
                        .short("u")
                        .long("unique")
                        .help("remove duplicates from an array")
                        .required(false),
                )
                .arg(
                    Arg::with_name("input")
                        .help("Input to process")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("capitalize")
                        .short("c")
                        .long("capitalize")
                        .help("Performs capitalization operations on the array.")
                        .takes_value(true)
                        .possible_values(&["camel", "pascal", "snake"])
                        .required(false),
                )
                // .arg(
                //     Arg::with_name("Array to props")
                //         .long("array_to_props")
                //         .help("Turn an array into React props to be used inside a component")
                //         .required(false),
                // )

        .subcommand(
            SubCommand::with_name("array_to_props")
                .help("Turn an array into React props to be used inside a component")
                .about("Turn an array into React props to be used inside a component")

    )
    )
        .subcommand(
            SubCommand::with_name("object")
                .about("perform operations on JSON like objects")
                .help("Perform operations on JSON like objects.")
                .arg(
                    Arg::with_name("difference")
                        .short("d")
                        .long("difference")
                        .help("show difference between two JSON like objects")
                        .required(false),
                )
                .arg(
                    Arg::with_name("object")
                        .help("Object to process")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("object_to_compare")
                        .help("Object to compare to.")
                        .index(2)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("array", Some(array_matches)) => with_array(array_matches),
        ("object", Some(object_matches)) => with_object(object_matches),
        ("", None) => println!("No subcommand used"),
        _ => println!("No known subcommand used",),
    }
}

fn array_to_props(matches: &ArgMatches, submatches: &ArgMatches, data: &Vec<&str>) {
    println!(
        "\n\nUsing array: \n{:?}, \n{:?}, \n{:?}",
        matches, submatches, data
    );
    // for elem in data {
    //     elem = &"2";
    // }
    // return data;
}

fn with_array(array_matches: &ArgMatches) {
    // if array_matches.is_present("array") {
    let data = array_matches.value_of("input").unwrap();
    let data_no_whitespaces = data.replace(", ", ",");
    let mut _data_as_array: Vec<&str> = data_no_whitespaces.split(',').collect();
    let mut data_as_array: Vec<String> = Vec::new();
    for d in &_data_as_array {
        let mut d_as_string = String::new();
        d_as_string.push_str(&d);
        data_as_array.push(d_as_string);
    }
    data_as_array.sort();

    // let mut array: Vec<_> =         .split(|c| c == ',' || c == ", ")
    //     .collect();
    println!("Matches data: {:?}\n\n", data_as_array);
    // }
    if array_matches.is_present("unique") {
        // println!("Matches: {:?}", array_matches);
        data_as_array.dedup();
        println!("Matches data: {:?}\n\n", data_as_array);
    }

    println!(
        "Matches data: {:?} \n\n",
        array_matches.is_present("capitalization")
    );
    if array_matches.is_present("capitalize") {
        // println!("Matches: {:?}", array_matches);
        // data_as_array.dedup();
        println!("Matches data: \n\n",);
        match array_matches.value_of("capitalize").unwrap() {
            "camel" => {
                for x in &data_as_array {
                    println!("{}\n", x);
                }
                println!("Camel")
            }
            "" => println!("None"),
            _ => println!("Error"),
        }
    }

    match array_matches.subcommand() {
        ("array_to_props", Some(array_submatches)) => {
            // array_to_props(&array_matches, array_submatches, &data_as_array);
            for elem in data_as_array.iter_mut() {
                println!("ELEM: {}", elem);
                let mut s = format!("{}={{{}}}", elem, elem);
                println!("{}", s);
                *elem = s;
            }
        }

        ("", None) => println!("No subcommand used"),
        _ => println!("No known subcommand used",),
    }

    println!("\n\nFinal: {:?} \n\n", data_as_array.join(" "));
}

fn is_different(
    object: &Map<String, Value>,
    object_to_compare: &Map<String, Value>,
    key: &String,
    object_differences: Map<String, Value>,
) -> Map<String, Value> {
    let mut new_diffs = object_differences;
    let key_original = &object.get(key);
    let key_to_compare = &object_to_compare.get(key);
    if key_to_compare.is_none() {
        new_diffs.insert(key.to_string().clone(), json!([object[key].clone(), null]));
        return new_diffs;
    }

    if !key_original.unwrap().is_array()
        && !key_original.unwrap().is_object()
        && key_original.unwrap() != key_to_compare.unwrap()
    {
        println!(
            "Chcking KEY: {}, {}, {}",
            key,
            key_original.unwrap().is_array(),
            key_original.unwrap().is_object()
        );
        new_diffs.insert(
            key.to_string().clone(),
            json!([object[key].clone(), object_to_compare[key].clone()]),
        );
        return new_diffs;
    }

    if key_original.unwrap().is_array() {}
    println!("A: {:?}, \n\nb: {:?}", object, object_to_compare);
    return new_diffs;
}
// fn is_different(value: &Value, value_to_compare: &Value) -> bool {
//     let mut is_diff = false;
//     println!("A: {:?}, b: {:?}", value, value_to_compare);
//     return is_diff;
// }

fn with_object(object_matches: &ArgMatches) {
    if object_matches.is_present("difference") {
        let mut object_differences = Map::new();
        let object = object_matches.value_of("object").unwrap();
        let parsed_object: Value = serde_json::from_str(object).unwrap();
        let o: Map<String, Value> = parsed_object.as_object().unwrap().clone();
        let object_compare = object_matches.value_of("object_to_compare").unwrap();
        let parsed_object_compare: Value = serde_json::from_str(object_compare).unwrap();
        let o_c: Map<String, Value> = parsed_object_compare.as_object().unwrap().clone();
        for key in o.keys() {
            object_differences = is_different(&o, &o_c, &key, object_differences);
            println!(
                "KEY: {:?}, {:?}  {:?}\n\n\n",
                key,
                o_c.get(key).is_none(),
                o_c.get(key).is_some(),
            );

            // if is_diff {
            // object_differences
            // .insert(key.to_string(), json!([o[key].clone(), o_c[key].clone()]));
            // }
        }
        // for key in o_c.keys() {
        //     let mut is_diff = false;
        //     if o.get(key).is_none() {
        //         is_diff = true;
        //     }
        //     if is_diff {
        //         object_differences.insert(key.to_string(), o_c[key].clone());
        //     }
        // }
        println!("object differences: {:?}", object_differences);
    }
}

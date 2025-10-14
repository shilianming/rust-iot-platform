/*
Copyright 2024 - 2025 Zen HuiFer

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

pub fn calc_bucket_name(prefix: &str, protocol: &str, id: i64) -> String {
    format!("{}_{}_{}", prefix, protocol, id % 100)
}


pub fn calc_collection_name(prefix: &str, id: i64) -> String {
    let string = format!("{}_{}", prefix, id % 100);
    return string;
}
pub fn calc_measurement(device_uid: &str, identification_code: &str, protocol: &str) -> String {
    format!("{}_{}_{}", protocol, device_uid, identification_code)
}

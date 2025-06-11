#[cfg(test)]
mod tests {
    use super::*;
    use crate::fivem_parser::FiveMDocParser;
    use std::path::PathBuf;

    #[test]
    fn test_manual_parse_dataarray_add_string_and_get_int() {
        let parser = FiveMDocParser::new(None, None);
        let md_add_string = r#"---
ns: DATAFILE
aliases: [\"_ARRAY_VALUE_ADD_STRING\"]
---
## DATAARRAY_ADD_STRING

```c
// 0x2F0661C155AEEEAA 0xF3C01350
void DATAARRAY_ADD_STRING(Any* arrayData, char* value);
```


## Parameters
* **arrayData**: 
* **value**: 

"#;
        let md_get_int = r#"---
ns: DATAFILE
aliases: [\"_ARRAY_VALUE_GET_INTEGER\"]
---
## DATAARRAY_GET_INT

```c
// 0x3E5AE19425CD74BE 0xBB120CFC
int DATAARRAY_GET_INT(Any* arrayData, int arrayIndex);
```


## Parameters
* **arrayData**: 
* **arrayIndex**: 

## Return value

"#;
        let res_add_string = parser.parse_native_from_markdown_content(md_add_string, "DATAFILE");
        println!("[DEBUG MANUAL PARSE] DATAARRAY_ADD_STRING: {:?}", res_add_string);
        let res_get_int = parser.parse_native_from_markdown_content(md_get_int, "DATAFILE");
        println!("[DEBUG MANUAL PARSE] DATAARRAY_GET_INT: {:?}", res_get_int);
    }
} 
# wasm-json-jsx

## 📦Description

Writing wasm-nodejs modules in Rust language can get much faster than javascript, converting JSON to JSX templates



## Example

```js
import * as wasm_module from "../pkg/wasm_json_jsx";
console.log(
  wasm_module.json_to_jsx(`{
    componentName: "root",
    children: [
      {
        componentName: "BoxComponent",
        type: "Box",
        currentId: "0",
        style: {
          left: "57px",
          top: "152px",
          width: "270px",
          height: "38px",
          border: "1px",
          backgroundColor: "#fb6f00",
          padding: "3px",
        },
        children: [
          {
            componentName: "TextComponent",
            type: "text",
            currentId: "1",
            data: "新建文本1",
            style: {
              left: "22px",
              top: "199px",
              width: "125px",
              height: "9px",
              border: "2px",
              backgroundColor: "#e36f40",
              padding: "2px",
              color: "#b12345",
              textAlign: "center",
              fontSize: "12px",
            },
          },
        ],
      },
      {
        componentName: "BoxComponent",
        type: "Box",
        currentId: "2",
        style: {
          left: "17px",
          top: "152px",
          width: "270px",
          height: "38px",
          border: "1px",
          backgroundColor: "#fb6f00",
          padding: "3px",
        },
        children: [
          {
            componentName: "ImageComponent",
            type: "text",
            currentId: "3",
            data: "https://***.jpg",
            style: {
              left: "22px",
              top: "199px",
              width: "125px",
              height: "9px",
              border: "2px",
              backgroundColor: "#e36f40",
              padding: "2px",
            },
          },
        ],
      },
    ],
  }`)
);
```



The above outputs:

```jsx
 <BoxComponent style = {{ backgroundColor="#fb6f00" border="1px" height="38px" left="57px" padding="3px" top="152px" width="270px"}} type="Box">
    <TextComponent data="新建文本1" style = {{ backgroundColor="#e36f40" border="2px" color="#b12345" fontSize="12px" height="9px" left="22px" padding="2px" textAlign="center" top="199px" width="125px"}} type="text">
    </TextComponent>
  </BoxComponent>
  <BoxComponent style = {{ backgroundColor="#fb6f00" border="1px" height="38px" left="17px" padding="3px" top="152px" width="270px"}} type="Box">
    <ImageComponent data="https://***.jpg" style = {{ backgroundColor="#e36f40" border="2px" height="9px" left="22px" padding="2px" top="199px" width="125px"}} type="text">
    </ImageComponent>
  </BoxComponent>
```


import * as wasm_module from "../pkg/wasm_json_jsx";
console.log(
  wasm_module.json_to_jsx(`{
    component_name: "name",
    a: 3,
    b: "ab",
    children: [
      {
        component_name: "child1",
        a: 3,
        b: "ab",
        innerText: "Hello World",
      },
      {
        component_name: "child2",
        a: 3,
        b: "ab",
        innerText: "Hello World",
      },
    ],
  }`)
);

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

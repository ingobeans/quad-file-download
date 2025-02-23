var ctx = null;
var memory;

params_set_mem = function (wasm_memory, _wasm_exports) {
  memory = wasm_memory;
  ctx = {};
};

params_register_js_plugin = function (importObject) {
  importObject.env.quad_file_download = function (pathObject, bytesObject) {
    let bytes = get_js_object(bytesObject);
    let path = get_js_object(pathObject);
    let mime = "application/octet-stream";
    var blob = new Blob([bytes], { type: mime });
    var objectUrl = URL.createObjectURL(blob);
    let a = document.createElement("a");
    a.href = objectUrl;
    a.download = path;
    a.click();
  };
};

miniquad_add_plugin({
  register_plugin: params_register_js_plugin,
  on_init: params_set_mem,
  name: "quad_file_download",
  version: "0.1.0",
});

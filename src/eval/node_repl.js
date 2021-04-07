// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.


const fs = require('fs');
var util = require('util');

var history = [];
var last_error = null;
var fress = null;

var _encoder = new util.TextEncoder();
var _decoder = new util.TextDecoder();
function into_utf8_bytes(s) { return _encoder.encode(s) }
function from_utf8_bytes(b) { return _decoder.decode(b) }

function log(byte_address, byte_count) {
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  var msg = from_utf8_bytes(slice);
  console.log(msg, 'font-size: 80%;')
}
function warn(byte_address, byte_count) {
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  console.warn(from_utf8_bytes(slice), 'font-size: 80%;')
}
function error(byte_address, byte_count) {
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  console.error(from_utf8_bytes(slice), 'font-size: 80%;')
}
function panic_error(byte_address, byte_count) {
  while (group_depth > 0) {
    group_depth -= 1;
    console.groupEnd();
  }
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  console.error(from_utf8_bytes(slice))
}
var group_depth = 0
function group_(msg) {
  group_depth += 1;
  console.groupCollapsed(msg)
}
function group(byte_address, byte_count) {
  group_depth += 1;
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  var msg = from_utf8_bytes(slice);
  console.groupCollapsed(msg, 'font-weight: normal; font-size: 80%;')
}
function group_end() {
  if (group_depth == 0) {
    throw "Not inside a console group, group_end call makes no sense.";
  } else {
    group_depth -= 1;
    console.groupEnd()
  }
}
function mark(byte_address, byte_count) {
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  performance.mark(from_utf8_bytes(slice))
}

function compile_init(byte_address, byte_count, mem_base, tab_base) {
  var module_bytes = new Uint8Array(fress.memory.buffer, byte_address, byte_count);
  fs.writeFile("comp.wasm", module_bytes);
  var module = new WebAssembly.Module(module_bytes);
  var im = {'fress': fress.instance.exports,
              'sys': {'memory': fress.memory,
                      'table': fress.table,
                      'memory_base': mem_base,
                      'table_base': tab_base}};
  var instance = new WebAssembly.Instance(module, im);
  history.push(module, instance);
  {
    group_('Module static_init');
    var zero = instance.exports.static_init();
    console.assert(zero == 0);
    group_end();
  }
  {
    group_('Module main');
    var res = instance.exports.main();
    group_end();
    fress.instance.exports.console_log(res)
  }
}

function post_output(byte_address, byte_count) {
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  var msg = from_utf8_bytes(slice);
  //
}
function post_error(byte_address, byte_count) {
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  var msg = from_utf8_bytes(slice);
  //
}

var console_imports =
  {'_console_log': log,
   '_console_warn': warn,
   '_console_error': error,
   '_console_panic_error': panic_error,
   '_console_group': group,
   '_console_group_end': group_end}
var performance_imports =
  {'_performance_mark': mark}
var env_imports =
  {'wasm_compile_init': compile_init,
   'post_output': post_output,
   'post_error': post_error}
var wasm_imports =
  {'env': env_imports,
   'console': console_imports,
   'performance': performance_imports}


function write_str(s) {
  var s_arr = into_utf8_bytes(s)
  new Uint8Array(fress.memory.buffer).set(s_arr); // starting at 0
  return s_arr.length
}

function e(s) {
  var len = write_str(s);
  try {
    fress.instance.exports.read_eval_print(0, len);
  }
  catch (err) {
    last_error = err;
  }
}
var ex = "(let [a 7] (conj [] a))";

WebAssembly.instantiate(fs.readFileSync("./fress.wasm"), wasm_imports)
.then(function (mod_inst) {
  var mod = mod_inst.module;
  var inst = mod_inst.instance;
  var exp = inst.exports;
  fress = {'module':   mod,
           'instance': inst,
           'memory':   exp.memory,
           'table':    exp.__indirect_function_table};
  exp.initialize_global_state();
  console.log("Ready!");
});




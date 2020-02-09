// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.

// Script for a Web Worker acting as a REPL
var history = []
var last_error = null
var fress = null

function error(byte_address, byte_count) {
  var decoder = new TextDecoder();
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  var str = decoder.decode(slice);
  console.error(str)
}
function log(byte_address, byte_count) {
  var decoder = new TextDecoder();
  var slice = fress.memory.buffer.slice(byte_address, byte_address + byte_count);
  var str = decoder.decode(slice);
  console.log(str)
}
function compile_init(byte_address, byte_count, mem_base, tab_base) {
  var module = new Uint8Array(fress.memory.buffer, byte_address, byte_count);
  var im = {'fress': fress.instance.exports,
            'sys': {'memory': fress.memory,
                    'table': fress.table,
                    'memory_base': mem_base,
                    'table_base': tab_base}}
  WebAssembly.instantiate(module, im).then(function (mod_inst) {
    history.push(mod_inst.module, mod_inst.instance);
    mod_inst.instance.exports.static_init();
    var res = mod_inst.instance.exports.main();
    fress.instance.exports.console_log(res)
  })
}
function write_str(s) {
  var encoder = new TextEncoder();
  var s_arr = encoder.encode(s);
  new Uint8Array(fress.memory.buffer).set(s_arr); // starting at 0
  return s_arr.length
}
function ev(msg) { handle_message(msg) }
function handle_message(msg) {
  var s = msg.data || msg;
  history.push(s)
  var len = write_str(s);
  try {
    fress.instance.exports.read_eval_print(0, len);
  }
  catch (err) {
    last_error = err;
  }
}

var sys_imports =
{'js_log_': log,
 'js_error_': error,
 'js_compile_init': compile_init}
WebAssembly.instantiateStreaming(fetch("fress.wasm"), {'cool_js': sys_imports})
.then(function (mod_inst) {
  var exp = mod_inst.instance.exports;
  var f = {'module': mod_inst.module,
           'instance': mod_inst.instance,
           'memory': exp.memory,
           'table': exp.__indirect_function_table}
  exp.initialize_global_state()
  fress = f
  history = []
  onmessage = handle_message;
}).then(function() { console.log("WASM loaded") });


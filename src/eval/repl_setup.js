// Copyright (c) Cole Frederick. All rights reserved.
// The use and distribution terms for this software are covered by the
// Eclipse Public License 1.0 (https://opensource.org/licenses/eclipse-1.0.php)
// which can be found in the file epl-v10.html at the root of this distribution.
// By using this software in any fashion, you are agreeing to be bound by the terms of this license.
// You must not remove this notice, or any other, from this software.


WebAssembly.compileStreaming(fetch("/fress.wasm"))
.then(function (mod) {
  console.log("Done compiling fress.wasm module!");
  document.fress_module = mod;
});

for (live of document.getElementsByClassName("live")) {
  let c = live.getElementsByTagName("pre")[0];
  c.classList.add("live-content");
  c.contentEditable = true;
  c.spellcheck = false;
  c.addEventListener("focus", first_focus);
}

function key_down(ev) {
  if (ev.repeat && !contains(["Backspace", "ArrowLeft", "ArrowRight"], ev.key)) {
    ev.preventDefault();
    ev.stopPropagation();
    return;
  }
  var x = (ev.ctrlKey ? "ctrl " : "") + (ev.altKey ? "alt " : "") +
       (ev.shiftKey ? "shift " : "") + (ev.metaKey ? "meta " : "");
  console.log("Key: " + ev.key + ", code: " + ev.code + " " + x);

  if (ev.key == "Enter" && ev.ctrlKey) {
    let live_elem = ev.target.closest('.live');
    let c = live_elem.getElementsByClassName("live-content")[0];
    live_elem.worker.postMessage(c.innerText.trim());
    ev.preventDefault();
    ev.stopPropagation();
    return;
  }
}
function contains(arr, elem) {
  return arr.indexOf(elem) != -1;
}
function on_paste(ev) {
  let p = ev.clipboardData.getData("text");
  console.log('Paste event |' + p + "|");
  let s = window.getSelection(); // Caret Range
  let r = s.getRangeAt(0);
  console.log(s.type + ", rangeCount: " + s.toString() + " " + s.isCollapsed);
  console.dir(s)
  ev.preventDefault();
  ev.stopPropagation();
}
function first_focus(ev) {
  if (!document.fress_module) { return }
  ev.target.removeEventListener("focus", first_focus);

  let content = ev.target;
  let live = content.closest('.live');
  content.addEventListener("keydown", key_down);
  content.addEventListener("paste", on_paste);

  var w = new Worker('/repl_worker.js');
  w.postMessage(document.fress_module);
  live.worker = w;
  add_button(live);
  w.onmessage = function (msg) { output_message(msg, live) };
}

function output_message(msg, live) {
  var m = document.createElement('pre');
  m.innerHTML = msg.data.output || msg.data.error;
  let o = output_div(live);
  o.appendChild(m);
  o.scrollTop = o.scrollHeight;
}

function output_div(live_elem) {
  var o = live_elem.getElementsByClassName("live-output");
  if (o.length != 0) {
    return o[0];
  } else {
    var o = document.createElement('div');
    o.classList.add("live-output");
    live_elem.insertBefore(o, live_elem.childNodes[0]);
    //live_elem.appendChild(o);
    return o;
  }
}

function add_button(live_elem) {
  var b = document.createElement('button');
  b.append('Run');
  b.classList.add('run-button');
  b.onclick = function(ev) {
    let c = live_elem.getElementsByClassName("live-content")[0];
    live_elem.worker.postMessage(c.innerText.trim());
  };
  live_elem.appendChild(b);
}

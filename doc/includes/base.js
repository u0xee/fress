
var pending_redraw = false;
var bar = document.getElementById('progress-bar');

function set_progress_width() {
  max_scroll = document.body.scrollHeight - window.innerHeight;
  percent = (window.scrollY / max_scroll) * 100;
  bar.style.width = percent.toFixed(1).concat("%");
  pending_redraw = false;
}
function sched_redraw () {
  if (!pending_redraw) {
    pending_redraw = true;
    window.requestAnimationFrame(set_progress_width);
  }
}

document.addEventListener('scroll', sched_redraw);

window.addEventListener("load", function () {
  sched_redraw();
  window.setTimeout( function() {
    document.documentElement.style.scrollBehavior = "smooth";
  }, 3000);
});


(function () {
  "use strict";

  function applyEffects(config) {
    if (!config || !config.elementos) return;

    config.elementos.forEach(function (elem) {
      var el = document.getElementById(elem.id);
      if (!el) return;

      if (elem.animacao) {
        var tipo = elem.animacao.tipo;
        var dur = elem.animacao.velocidade || 4;
        var amp = elem.animacao.intensidade || 5;

        if (tipo === "pulsar") {
          applyPulsar(el, dur, amp);
        } else if (tipo === "deriva") {
          applyDeriva(el, dur, amp);
        } else if (tipo === "aparecer") {
          applyAparecer(el, dur);
        } else if (tipo === "impulso") {
          applyImpulso(el, dur);
        }
      }
    });
  }

  function applyPulsar(el, dur, amp) {
    var start = null;
    var rx0 = parseFloat(el.getAttribute("rx") || el.getAttribute("r") || 0);
    var ry0 = parseFloat(el.getAttribute("ry") || el.getAttribute("r") || 0);

    function tick(ts) {
      if (!start) start = ts;
      var t = ((ts - start) / 1000) % dur;
      var phase = (t / dur) * Math.PI * 2;
      var delta = Math.sin(phase) * amp * 0.5;

      if (el.hasAttribute("rx")) {
        el.setAttribute("rx", rx0 + delta);
        el.setAttribute("ry", ry0 - delta * 0.7);
      } else if (el.hasAttribute("r")) {
        el.setAttribute("r", rx0 + delta);
      } else {
        el.setAttribute("transform", "scale(" + (1 + delta * 0.01) + ")");
      }
      requestAnimationFrame(tick);
    }
    requestAnimationFrame(tick);
  }

  function applyDeriva(el, dur, amp) {
    var start = null;
    var transform0 = el.getAttribute("transform") || "";

    function tick(ts) {
      if (!start) start = ts;
      var t = ((ts - start) / 1000) % dur;
      var phase = (t / dur) * Math.PI * 2;
      var dx = Math.sin(phase) * amp;
      var dy = Math.cos(phase * 0.7) * amp * 0.6;
      el.setAttribute("transform", transform0 + " translate(" + dx + "," + dy + ")");
      requestAnimationFrame(tick);
    }
    requestAnimationFrame(tick);
  }

  function applyAparecer(el, dur) {
    var start = null;

    function tick(ts) {
      if (!start) start = ts;
      var t = ((ts - start) / 1000) % dur;
      var phase = (t / dur) * Math.PI * 2;
      var opacity = (Math.sin(phase) + 1) / 2;
      el.style.opacity = opacity;
      requestAnimationFrame(tick);
    }
    requestAnimationFrame(tick);
  }

  function applyImpulso(el, dur) {
    var start = null;

    function tick(ts) {
      if (!start) start = ts;
      var t = ((ts - start) / 1000) % dur;
      var phase = t / dur;
      var opacity = phase < 0.5
        ? phase * 2
        : 1 - (phase - 0.5) * 2;
      el.style.opacity = opacity;
      requestAnimationFrame(tick);
    }
    requestAnimationFrame(tick);
  }

  function init() {
    var configEl = document.getElementById("edusvg-config");
    if (!configEl) return;

    try {
      var config = JSON.parse(configEl.textContent);
      applyEffects(config);
    } catch (e) {
      console.warn("EduSVG: erro ao ler configuracao.", e);
    }
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();

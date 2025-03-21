(() => {
  'use strict';
  var e,
    t,
    r,
    a,
    o,
    n = {},
    i = {};
  function c(e) {
    var t = i[e];
    if (void 0 !== t) return t.exports;
    var r = (i[e] = { id: e, loaded: !1, exports: {} });
    return n[e].call(r.exports, r, r.exports, c), (r.loaded = !0), r.exports;
  }
  (c.m = n),
    (c.c = i),
    (e = []),
    (c.O = (t, r, a, o) => {
      if (!r) {
        var n = 1 / 0;
        for (f = 0; f < e.length; f++) {
          (r = e[f][0]), (a = e[f][1]), (o = e[f][2]);
          for (var i = !0, d = 0; d < r.length; d++)
            (!1 & o || n >= o) && Object.keys(c.O).every((e) => c.O[e](r[d]))
              ? r.splice(d--, 1)
              : ((i = !1), o < n && (n = o));
          if (i) {
            e.splice(f--, 1);
            var u = a();
            void 0 !== u && (t = u);
          }
        }
        return t;
      }
      o = o || 0;
      for (var f = e.length; f > 0 && e[f - 1][2] > o; f--) e[f] = e[f - 1];
      e[f] = [r, a, o];
    }),
    (c.n = (e) => {
      var t = e && e.__esModule ? () => e.default : () => e;
      return c.d(t, { a: t }), t;
    }),
    (r = Object.getPrototypeOf ? (e) => Object.getPrototypeOf(e) : (e) => e.__proto__),
    (c.t = function (e, a) {
      if ((1 & a && (e = this(e)), 8 & a)) return e;
      if ('object' == typeof e && e) {
        if (4 & a && e.__esModule) return e;
        if (16 & a && 'function' == typeof e.then) return e;
      }
      var o = Object.create(null);
      c.r(o);
      var n = {};
      t = t || [null, r({}), r([]), r(r)];
      for (var i = 2 & a && e; 'object' == typeof i && !~t.indexOf(i); i = r(i))
        Object.getOwnPropertyNames(i).forEach((t) => (n[t] = () => e[t]));
      return (n.default = () => e), c.d(o, n), o;
    }),
    (c.d = (e, t) => {
      for (var r in t) c.o(t, r) && !c.o(e, r) && Object.defineProperty(e, r, { enumerable: !0, get: t[r] });
    }),
    (c.f = {}),
    (c.e = (e) => Promise.all(Object.keys(c.f).reduce((t, r) => (c.f[r](e, t), t), []))),
    (c.u = (e) =>
      'assets/js/' +
      ({
        48: 'a94703ab',
        98: 'a7bd4aaa',
        235: 'a7456010',
        384: 'a8e3f558',
        401: '17896441',
        583: '1df93b7f',
        647: '5e95c892',
        742: 'aba21aa0',
        803: '3b8c55ea',
      }[e] || e) +
      '.' +
      {
        48: 'ef69ea97',
        74: 'f4289094',
        98: '57fd031e',
        235: '47cad1bc',
        384: '6c0760c1',
        401: '85c9e21d',
        583: 'c8eaaf9c',
        647: '2e902663',
        742: 'eb7bf6f2',
        803: '89de96f2',
      }[e] +
      '.js'),
    (c.miniCssF = (e) => {}),
    (c.g = (function () {
      if ('object' == typeof globalThis) return globalThis;
      try {
        return this || new Function('return this')();
      } catch (e) {
        if ('object' == typeof window) return window;
      }
    })()),
    (c.o = (e, t) => Object.prototype.hasOwnProperty.call(e, t)),
    (a = {}),
    (o = 'docs:'),
    (c.l = (e, t, r, n) => {
      if (a[e]) a[e].push(t);
      else {
        var i, d;
        if (void 0 !== r)
          for (var u = document.getElementsByTagName('script'), f = 0; f < u.length; f++) {
            var l = u[f];
            if (l.getAttribute('src') == e || l.getAttribute('data-webpack') == o + r) {
              i = l;
              break;
            }
          }
        i ||
          ((d = !0),
          ((i = document.createElement('script')).charset = 'utf-8'),
          (i.timeout = 120),
          c.nc && i.setAttribute('nonce', c.nc),
          i.setAttribute('data-webpack', o + r),
          (i.src = e)),
          (a[e] = [t]);
        var s = (t, r) => {
            (i.onerror = i.onload = null), clearTimeout(b);
            var o = a[e];
            if ((delete a[e], i.parentNode && i.parentNode.removeChild(i), o && o.forEach((e) => e(r)), t)) return t(r);
          },
          b = setTimeout(s.bind(null, void 0, { type: 'timeout', target: i }), 12e4);
        (i.onerror = s.bind(null, i.onerror)), (i.onload = s.bind(null, i.onload)), d && document.head.appendChild(i);
      }
    }),
    (c.r = (e) => {
      'undefined' != typeof Symbol &&
        Symbol.toStringTag &&
        Object.defineProperty(e, Symbol.toStringTag, { value: 'Module' }),
        Object.defineProperty(e, '__esModule', { value: !0 });
    }),
    (c.p = '/codora/'),
    (c.gca = function (e) {
      return (
        (e =
          {
            17896441: '401',
            a94703ab: '48',
            a7bd4aaa: '98',
            a7456010: '235',
            a8e3f558: '384',
            '1df93b7f': '583',
            '5e95c892': '647',
            aba21aa0: '742',
            '3b8c55ea': '803',
          }[e] || e),
        c.p + c.u(e)
      );
    }),
    (() => {
      var e = { 354: 0, 869: 0 };
      (c.f.j = (t, r) => {
        var a = c.o(e, t) ? e[t] : void 0;
        if (0 !== a)
          if (a) r.push(a[2]);
          else if (/^(354|869)$/.test(t)) e[t] = 0;
          else {
            var o = new Promise((r, o) => (a = e[t] = [r, o]));
            r.push((a[2] = o));
            var n = c.p + c.u(t),
              i = new Error();
            c.l(
              n,
              (r) => {
                if (c.o(e, t) && (0 !== (a = e[t]) && (e[t] = void 0), a)) {
                  var o = r && ('load' === r.type ? 'missing' : r.type),
                    n = r && r.target && r.target.src;
                  (i.message = 'Loading chunk ' + t + ' failed.\n(' + o + ': ' + n + ')'),
                    (i.name = 'ChunkLoadError'),
                    (i.type = o),
                    (i.request = n),
                    a[1](i);
                }
              },
              'chunk-' + t,
              t
            );
          }
      }),
        (c.O.j = (t) => 0 === e[t]);
      var t = (t, r) => {
          var a,
            o,
            n = r[0],
            i = r[1],
            d = r[2],
            u = 0;
          if (n.some((t) => 0 !== e[t])) {
            for (a in i) c.o(i, a) && (c.m[a] = i[a]);
            if (d) var f = d(c);
          }
          for (t && t(r); u < n.length; u++) (o = n[u]), c.o(e, o) && e[o] && e[o][0](), (e[o] = 0);
          return c.O(f);
        },
        r = (self.webpackChunkdocs = self.webpackChunkdocs || []);
      r.forEach(t.bind(null, 0)), (r.push = t.bind(null, r.push.bind(r)));
    })();
})();

(()=>{"use strict";var e={m:{},u:e=>e+".index.js"};e.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),e.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),(()=>{var t;e.g.importScripts&&(t=e.g.location+"");var n=e.g.document;if(!t&&n&&(n.currentScript&&(t=n.currentScript.src),!t)){var r=n.getElementsByTagName("script");r.length&&(t=r[r.length-1].src)}if(!t)throw new Error("Automatic publicPath is not supported in this browser");t=t.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),e.p=t})(),e.b=document.baseURI||self.location.href;const t=document.getElementById("canvas"),{width:n,height:r}=t,a=t.getContext("2d"),s=document.getElementById("time"),i=new Worker(new URL(new URL(e.p+e.u(979),e.b)),{type:void 0});i.onmessage=({data:{rawImageData:e,time:t}})=>{s.value=`${t.toFixed(2)} ms`;const i=new ImageData(e,n,r);a.putImageData(i,0,0)};const o={onclick(e){i.postMessage({type:e.target.id,width:n,height:r,maxIterations:1e3})},disabled:!1};Object.assign(document.getElementById("singleThread"),o),(async e=>{try{return"undefined"!=typeof MessageChannel&&(new MessageChannel).port1.postMessage(new SharedArrayBuffer(1)),WebAssembly.validate(e)}catch(e){return!1}})(new Uint8Array([0,97,115,109,1,0,0,0,1,4,1,96,0,0,3,2,1,0,5,4,1,3,1,1,10,11,1,9,0,65,0,254,16,2,0,26,11])).then((e=>{e&&Object.assign(document.getElementById("multiThread"),o)}))})();
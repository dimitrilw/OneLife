!function(t){var e={};function a(r){if(e[r])return e[r].exports;var n=e[r]={i:r,l:!1,exports:{}};return t[r].call(n.exports,n,n.exports,a),n.l=!0,n.exports}a.m=t,a.c=e,a.d=function(t,e,r){a.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:r})},a.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},a.t=function(t,e){if(1&e&&(t=a(t)),8&e)return t;if(4&e&&"object"==typeof t&&t&&t.__esModule)return t;var r=Object.create(null);if(a.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var n in t)a.d(r,n,function(e){return t[e]}.bind(null,n));return r},a.n=function(t){var e=t&&t.__esModule?function(){return t.default}:function(){return t};return a.d(e,"a",e),e},a.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},a.p="",a(a.s=25)}({25:function(t,e){!function(t){if(!/MSIE 10/i.test(navigator.userAgent))try{var e=new Blob(["var fakeIdToId = {};onmessage = function (event) {\tvar data = event.data,\t\tname = data.name,\t\tfakeId = data.fakeId,\t\ttime;\tif(data.hasOwnProperty('time')) {\t\ttime = data.time;\t}\tswitch (name) {\t\tcase 'setInterval':\t\t\tfakeIdToId[fakeId] = setInterval(function () {\t\t\t\tpostMessage({fakeId: fakeId});\t\t\t}, time);\t\t\tbreak;\t\tcase 'clearInterval':\t\t\tif (fakeIdToId.hasOwnProperty (fakeId)) {\t\t\t\tclearInterval(fakeIdToId[fakeId]);\t\t\t\tdelete fakeIdToId[fakeId];\t\t\t}\t\t\tbreak;\t\tcase 'setTimeout':\t\t\tfakeIdToId[fakeId] = setTimeout(function () {\t\t\t\tpostMessage({fakeId: fakeId});\t\t\t\tif (fakeIdToId.hasOwnProperty (fakeId)) {\t\t\t\t\tdelete fakeIdToId[fakeId];\t\t\t\t}\t\t\t}, time);\t\t\tbreak;\t\tcase 'clearTimeout':\t\t\tif (fakeIdToId.hasOwnProperty (fakeId)) {\t\t\t\tclearTimeout(fakeIdToId[fakeId]);\t\t\t\tdelete fakeIdToId[fakeId];\t\t\t}\t\t\tbreak;\t}}"]);t=window.URL.createObjectURL(e)}catch(t){}var a,r={},n=0,o="HackTimer.js by turuslan: ";if("undefined"!=typeof Worker){function i(){do{2147483647==n?n=0:n++}while(r.hasOwnProperty(n));return n}try{a=new Worker(t),window.setInterval=function(t,e){var n=i();return r[n]={callback:t,parameters:Array.prototype.slice.call(arguments,2)},a.postMessage({name:"setInterval",fakeId:n,time:e}),n},window.clearInterval=function(t){r.hasOwnProperty(t)&&(delete r[t],a.postMessage({name:"clearInterval",fakeId:t}))},window.setTimeout=function(t,e){var n=i();return r[n]={callback:t,parameters:Array.prototype.slice.call(arguments,2),isTimeout:!0},a.postMessage({name:"setTimeout",fakeId:n,time:e}),n},window.clearTimeout=function(t){r.hasOwnProperty(t)&&(delete r[t],a.postMessage({name:"clearTimeout",fakeId:t}))},a.onmessage=function(t){var e,a,n,i=t.data.fakeId;if(r.hasOwnProperty(i)&&(n=(e=r[i]).callback,a=e.parameters,e.hasOwnProperty("isTimeout")&&e.isTimeout&&delete r[i]),"string"==typeof n)try{n=new Function(n)}catch(t){console.log(o+"Error parsing callback code string: ",t)}"function"==typeof n&&n.apply(window,a)},a.onerror=function(t){console.log(t)}}catch(t){console.log(o+"Initialisation failed"),console.error(t)}}else console.log(o+"Initialisation failed - HTML5 Web Worker is not supported")}("HackTimerWorker.js")}});
let R=0,U=`string`,T=1,W=`Object`,P=`utf-8`,N=null,O=`undefined`,Y=4,V=`function`,L=128,K=Array,Q=Error,X=FinalizationRegistry,Z=Object,S=Uint8Array,M=undefined;var E=(async(a,b)=>{if(typeof Response===V&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===V){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var l=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==N){return `${a}`};if(b==U){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==N){return `Symbol`}else{return `Symbol(${b})`}};if(b==V){const b=a.name;if(typeof b==U&&b.length>R){return `Function(${b})`}else{return `Function`}};if(K.isArray(a)){const b=a.length;let c=`[`;if(b>R){c+=l(a[R])};for(let d=T;d<b;d++){c+=`, `+ l(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>T){d=c[T]}else{return toString.call(a)};if(d==W){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return W}};if(a instanceof Q){return `${a.name}: ${a.message}\n${a.stack}`};return d});var w=((c,d,e)=>{try{a._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcc675085f38c412d(c,d,v(e))}finally{b[u++]=M}});var G=((a,b)=>{});var D=((a,b)=>{a=a>>>R;const c=C();const d=c.subarray(a/Y,a/Y+ b);const e=[];for(let a=R;a<d.length;a++){e.push(f(d[a]))};return e});var k=(a=>{if(d===b.length)b.push(b.length+ T);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});function A(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var J=(async(b)=>{if(a!==M)return a;if(typeof b===O){b=new URL(`konnektoren-pwa-657fb96728e23d6d_bg.wasm`,import.meta.url)};const c=F();if(typeof b===U||typeof Request===V&&b instanceof Request||typeof URL===V&&b instanceof URL){b=fetch(b)};G(c);const {instance:d,module:e}=await E(await b,c);return H(d,e)});var I=(b=>{if(a!==M)return a;const c=F();G(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return H(d,b)});var r=(()=>{if(q===N||q.byteLength===R){q=new Int32Array(a.memory.buffer)};return q});var z=(a=>a===M||a===N);var c=(a=>b[a]);var H=((b,c)=>{a=b.exports;J.__wbindgen_wasm_module=c;q=N;B=N;h=N;a.__wbindgen_start();return a});var F=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==T){b.a=R;return !0};const c=!1;return c});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/Y+ T]=z(d)?R:d;r()[a/Y+ R]=!z(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>R});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/Y+ T]=z(d)?R:d;r()[a/Y+ R]=!z(d)});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>R});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>R});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/Y+ T]=z(d)?R:d;r()[a/Y+ R]=!z(d)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new Q();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,T)}});b.wbg.__wbg_queueMicrotask_f61ee94ee663068b=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_f82fc5d1e8f816ae=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===V;return b});b.wbg.__wbg_crypto_d05b68a3572bb8ca=(a=>{const b=c(a).crypto;return k(b)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==N;return d});b.wbg.__wbg_process_b02b3570280d0366=(a=>{const b=c(a).process;return k(b)});b.wbg.__wbg_versions_c1cb42213cedf0f5=(a=>{const b=c(a).versions;return k(b)});b.wbg.__wbg_node_43b1089f407e4ec2=(a=>{const b=c(a).node;return k(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===U;return b});b.wbg.__wbg_msCrypto_10fc94afee92bd76=(a=>{const b=c(a).msCrypto;return k(b)});b.wbg.__wbg_require_9a7e0f667ead4995=function(){return A((()=>{const a=module.require;return k(a)}),arguments)};b.wbg.__wbg_randomFillSync_b70ccbdf4926a99d=function(){return A(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d=function(){return A(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=D(b,c).slice();a.__wbindgen_free(b,c*Y,Y);console.error(...d)});b.wbg.__wbg_warn_71afa7f8150659a1=((b,c)=>{var d=D(b,c).slice();a.__wbindgen_free(b,c*Y,Y);console.warn(...d)});b.wbg.__wbg_body_874ccb42daaab363=(a=>{const b=c(a).body;return z(b)?R:k(b)});b.wbg.__wbg_createElement_03cf347ddad1c8c0=function(){return A(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_93f8de4acdef6da8=function(){return A(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===R?M:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_ea32ad2506f7ae78=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_instanceof_Element_813f33306edae612=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_230708ae7f4baac5=((b,d)=>{const e=c(d).namespaceURI;var f=z(e)?R:p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbg_setinnerHTML_95222f1a2e797983=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_eb21059e86b1e9f4=((b,d)=>{const e=c(d).outerHTML;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbg_removeAttribute_0c021c98a4dc7402=function(){return A(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_setAttribute_f7ffa687ef977957=function(){return A(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_instanceof_Window_cee7a886d55e7df5=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_eb7fd66bde3ee213=(a=>{const b=c(a).document;return z(b)?R:k(b)});b.wbg.__wbg_bubbles_31126fc08276cf99=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_ae95595adf5ae83d=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_bd8a0336a042e053=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_instanceof_ShadowRoot_ef56f954a86c7472=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_dfffc3b2ba786fb8=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_addEventListener_bc4a7ad4cc72c6bf=function(){return A(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_setchecked_50e21357d62a8ccd=((a,b)=>{c(a).checked=b!==R});b.wbg.__wbg_value_99f5294791d62576=((b,d)=>{const e=c(d).value;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbg_setvalue_bba31de32cdbb32c=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_parentNode_e3a5ee563364a613=(a=>{const b=c(a).parentNode;return z(b)?R:k(b)});b.wbg.__wbg_parentElement_45a9756dc74ff48b=(a=>{const b=c(a).parentElement;return z(b)?R:k(b)});b.wbg.__wbg_childNodes_535387effca84f4e=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_d22dbf81f92f163b=(a=>{const b=c(a).lastChild;return z(b)?R:k(b)});b.wbg.__wbg_nextSibling_87d2b32dfbf09fe3=(a=>{const b=c(a).nextSibling;return z(b)?R:k(b)});b.wbg.__wbg_setnodeValue_d1cec51282858afe=((a,b,d)=>{c(a).nodeValue=b===R?M:j(b,d)});b.wbg.__wbg_textContent_528ff517a0418a3e=((b,d)=>{const e=c(d).textContent;var f=z(e)?R:p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbg_cloneNode_ea49a704f0699b2e=function(){return A((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_2be91083083caa9e=function(){return A(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_660924798c7e128c=function(){return A(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_value_ffef403d62e3df58=((b,d)=>{const e=c(d).value;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbg_setvalue_cbab536654d8dd52=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_get_0ee8ea3c7c984c45=((a,b)=>{const d=c(a)[b>>>R];return k(d)});b.wbg.__wbg_length_161c0d89c6535c1d=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_cfecb3965268594c=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_3f093dd26d5569f8=function(){return A(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_632630b5cec17f21=(()=>{const a=new Z();return k(a)});b.wbg.__wbg_self_05040bd9523805b9=function(){return A((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_adc720039f2cb14f=function(){return A((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_622105db80c1457d=function(){return A((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f56b013ed9bcf359=function(){return A((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===M;return b});b.wbg.__wbg_from_58c79ccfb68060f5=(a=>{const b=K.from(c(a));return k(b)});b.wbg.__wbg_call_67f2111acd2dfdb6=function(){return A(((a,b,d)=>{const e=c(a).call(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_is_bd5dc4ae269cba1c=((a,b)=>{const d=Z.is(c(a),c(b));return d});b.wbg.__wbg_resolve_5da6faf2c96fd1d5=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_f9e58f5a50f43eae=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_buffer_b914fb8b50ebbc3e=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_0de9ee56e9f6ee6e=((a,b,d)=>{const e=new S(c(a),b>>>R,d>>>R);return k(e)});b.wbg.__wbg_new_b1f2d6842d615181=(a=>{const b=new S(c(a));return k(b)});b.wbg.__wbg_set_7d988c98e6ced92d=((a,b,d)=>{c(a).set(c(b),d>>>R)});b.wbg.__wbg_newwithlength_0d03cef43b68a530=(a=>{const b=new S(a>>>R);return k(b)});b.wbg.__wbg_subarray_adc418253d76e2f1=((a,b,d)=>{const e=c(a).subarray(b>>>R,d>>>R);return k(e)});b.wbg.__wbg_set_961700853a212a39=function(){return A(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=l(c(d));const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/Y+ T]=g;r()[b/Y+ R]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new Q(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper2513=((a,b,c)=>{const d=t(a,b,1321,w);return k(d)});b.wbg.__wbindgen_closure_wrapper2890=((a,b,c)=>{const d=x(a,b,1438,y);return k(d)});return b});var C=(()=>{if(B===N||B.byteLength===R){B=new Uint32Array(a.memory.buffer)};return B});var y=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3ae89a6bef83f1c(b,c,k(d))});var t=((b,c,d,e)=>{const f={a:b,b:c,cnt:T,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===R){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=R;s.unregister(f)}}};g.original=f;s.register(g,f,f);return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var x=((b,c,d,e)=>{const f={a:b,b:c,cnt:T,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=R;try{return e(c,f.b,...b)}finally{if(--f.cnt===R){a.__wbindgen_export_2.get(f.dtor)(c,f.b);s.unregister(f)}else{f.a=c}}};g.original=f;s.register(g,f,f);return g});var p=((a,b,c)=>{if(c===M){const c=n.encode(a);const d=b(c.length,T)>>>R;i().subarray(d,d+ c.length).set(c);m=c.length;return d};let d=a.length;let e=b(d,T)>>>R;const f=i();let g=R;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==R){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,T)>>>R;const b=i().subarray(e+ g,e+ d);const f=o(a,b);g+=f.written;e=c(e,d,g,T)>>>R};m=g;return e});var i=(()=>{if(h===N||h.byteLength===R){h=new S(a.memory.buffer)};return h});var j=((a,b)=>{a=a>>>R;return g.decode(i().subarray(a,a+ b))});var v=(a=>{if(u==T)throw new Q(`out of js stack`);b[--u]=a;return u});let a;const b=new K(L).fill(M);b.push(M,N,!0,!1);let d=b.length;const g=typeof TextDecoder!==O?new TextDecoder(P,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Q(`TextDecoder not available`)}};if(typeof TextDecoder!==O){g.decode()};let h=N;let m=R;const n=typeof TextEncoder!==O?new TextEncoder(P):{encode:()=>{throw Q(`TextEncoder not available`)}};const o=typeof n.encodeInto===V?((a,b)=>n.encodeInto(a,b)):((a,b)=>{const c=n.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=N;const s=typeof X===O?{register:()=>{},unregister:()=>{}}:new X(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let u=L;let B=N;export default J;export{I as initSync}
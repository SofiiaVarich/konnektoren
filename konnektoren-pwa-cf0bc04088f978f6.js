let V=1,W=0,a1=`boolean`,U=null,a0=`number`,$=`function`,a2=`string`,Y=`undefined`,Z=`utf-8`,S=128,a8=1748,a3=`Object`,a6=4,R=Array,_=Error,a5=FinalizationRegistry,a4=JSON.stringify,a7=Object,X=Uint8Array,T=undefined;var v=(a=>{const b=typeof a;if(b==a0||b==a1||a==U){return `${a}`};if(b==a2){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==U){return `Symbol`}else{return `Symbol(${b})`}};if(b==$){const b=a.name;if(typeof b==a2&&b.length>W){return `Function(${b})`}else{return `Function`}};if(R.isArray(a)){const b=a.length;let c=`[`;if(b>W){c+=v(a[W])};for(let d=V;d<b;d++){c+=`, `+ v(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>V){d=c[V]}else{return toString.call(a)};if(d==a3){try{return `Object(`+ a4(a)+ `)`}catch(a){return a3}};if(a instanceof _){return `${a.name}: ${a.message}\n${a.stack}`};return d});var g=(a=>{const b=d(a);f(a);return b});var O=((a,c)=>{b=a.exports;Q.__wbindgen_wasm_module=c;t=U;p=U;I=U;j=U;b.__wbindgen_start();return b});var n=((a,b,c)=>{if(c===T){const c=l.encode(a);const d=b(c.length,V)>>>W;k().subarray(d,d+ c.length).set(c);i=c.length;return d};let d=a.length;let e=b(d,V)>>>W;const f=k();let g=W;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==W){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,V)>>>W;const b=k().subarray(e+ g,e+ d);const f=m(a,b);g+=f.written;e=c(e,d,g,V)>>>W};i=g;return e});var d=(a=>c[a]);var f=(a=>{if(a<132)return;c[a]=e;e=a});var F=((a,c,d)=>{b._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6a339328e19c29d0(a,c,h(d))});var P=(a=>{if(b!==T)return b;const c=M();N(c);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const d=new WebAssembly.Instance(a,c);return O(d,a)});var k=(()=>{if(j===U||j.byteLength===W){j=new X(b.memory.buffer)};return j});function H(a,c){try{return a.apply(this,c)}catch(a){b.__wbindgen_exn_store(h(a))}}var z=((a,c,d)=>{b.wasm_bindgen__convert__closures__invoke1_mut__h9292b468a1b107b7(a,c,h(d))});var N=((a,b)=>{});var y=((a,c)=>{b.wasm_bindgen__convert__closures__invoke0_mut__hc9ae5cc20a9521ae(a,c)});var J=(()=>{if(I===U||I.byteLength===W){I=new Uint32Array(b.memory.buffer)};return I});var A=((a,c,d)=>{b.wasm_bindgen__convert__closures__invoke1_mut__h23bb8c200f354467(a,c,h(d))});var L=(async(a,b)=>{if(typeof Response===$&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===$){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var s=((a,b)=>{a=a>>>W;return r.decode(k().subarray(a,a+ b))});var o=(a=>a===T||a===U);var u=(()=>{if(t===U||t.byteLength===W){t=new Float64Array(b.memory.buffer)};return t});var G=((a,d,e)=>{try{b._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3416cfd6c6ddde82(a,d,D(e))}finally{c[C++]=T}});var D=(a=>{if(C==V)throw new _(`out of js stack`);c[--C]=a;return C});var E=((a,d,e)=>{try{b._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0b4f6f5ba634cc4e(a,d,D(e))}finally{c[C++]=T}});var h=(a=>{if(e===c.length)c.push(c.length+ V);const b=e;e=c[b];c[b]=a;return b});var M=(()=>{const c={};c.wbg={};c.wbg.__wbindgen_object_drop_ref=(a=>{g(a)});c.wbg.__wbindgen_cb_drop=(a=>{const b=g(a).original;if(b.cnt--==V){b.a=W;return !0};const c=!1;return c});c.wbg.__wbindgen_object_clone_ref=(a=>{const b=d(a);return h(b)});c.wbg.__wbg_connectSolanaWallet_f166a395e15a3e34=(()=>{const b=a();return h(b)});c.wbg.__wbindgen_is_undefined=(a=>{const b=d(a)===T;return b});c.wbg.__wbindgen_is_null=(a=>{const b=d(a)===U;return b});c.wbg.__wbindgen_string_get=((a,c)=>{const e=d(c);const f=typeof e===a2?e:T;var g=o(f)?W:n(f,b.__wbindgen_malloc,b.__wbindgen_realloc);var h=i;q()[a/a6+ V]=h;q()[a/a6+ W]=g});c.wbg.__wbindgen_string_new=((a,b)=>{const c=s(a,b);return h(c)});c.wbg.__wbg_instanceof_ClipboardItem_f68dca2b15f12ca4=(a=>{let b;try{b=d(a) instanceof ClipboardItem}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_instanceof_Window_6faa18fbaad60885=(a=>{let b;try{b=d(a) instanceof Window}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_read_f3915f795a505cff=(a=>{const b=d(a).read();return h(b)});c.wbg.__wbg_readText_57eee82b45ff69bf=(a=>{const b=d(a).readText();return h(b)});c.wbg.__wbg_write_ea7f24b79f380473=((a,b)=>{const c=d(a).write(d(b));return h(c)});c.wbg.__wbg_writeText_168aa980f3ae6365=((a,b,c)=>{const e=d(a).writeText(s(b,c));return h(e)});c.wbg.__wbg_clipboard_c45a65d94018d435=(a=>{const b=d(a).clipboard;return o(b)?W:h(b)});c.wbg.__wbg_navigator_5e4972bacd50a0cb=(a=>{const b=d(a).navigator;return h(b)});c.wbg.__wbg_new_859e9d4be18d3aaa=function(){return H((a=>{const b=new ClipboardItem(d(a));return h(b)}),arguments)};c.wbg.__wbg_types_dfe555355d13be1e=(a=>{const b=d(a).types;return h(b)});c.wbg.__wbg_getType_d8891fee06a3d3fd=((a,b,c)=>{const e=d(a).getType(s(b,c));return h(e)});c.wbg.__wbindgen_is_function=(a=>{const b=typeof d(a)===$;return b});c.wbg.__wbindgen_is_string=(a=>{const b=typeof d(a)===a2;return b});c.wbg.__wbindgen_number_get=((a,b)=>{const c=d(b);const e=typeof c===a0?c:T;u()[a/8+ V]=o(e)?W:e;q()[a/a6+ W]=!o(e)});c.wbg.__wbindgen_number_new=(a=>{const b=a;return h(b)});c.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const c=d(b).__yew_subtree_cache_key;q()[a/a6+ V]=o(c)?W:c;q()[a/a6+ W]=!o(c)});c.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const c=d(b).__yew_subtree_id;q()[a/a6+ V]=o(c)?W:c;q()[a/a6+ W]=!o(c)});c.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{d(a).__yew_subtree_id=b>>>W});c.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{d(a).__yew_subtree_cache_key=b>>>W});c.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const c=d(b).__yew_listener_id;q()[a/a6+ V]=o(c)?W:c;q()[a/a6+ W]=!o(c)});c.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{d(a).__yew_listener_id=b>>>W});c.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new _();return h(a)});c.wbg.__wbg_stack_658279fe44541cf6=((a,c)=>{const e=d(c).stack;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_error_f851667af71bcfc6=((a,c)=>{let d;let e;try{d=a;e=c;console.error(s(a,c))}finally{b.__wbindgen_free(d,e,V)}});c.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return H(((a,b)=>{const c=setTimeout(d(a),b);return h(c)}),arguments)};c.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(d(a))});c.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=d(a).queueMicrotask;return h(b)});c.wbg.__wbindgen_is_object=(a=>{const b=d(a);const c=typeof b===`object`&&b!==U;return c});c.wbg.__wbindgen_in=((a,b)=>{const c=d(a) in d(b);return c});c.wbg.__wbindgen_error_new=((a,b)=>{const c=new _(s(a,b));return h(c)});c.wbg.__wbg_crypto_566d7465cdbb6b7a=(a=>{const b=d(a).crypto;return h(b)});c.wbg.__wbg_process_dc09a8c7d59982f6=(a=>{const b=d(a).process;return h(b)});c.wbg.__wbg_versions_d98c6400c6ca2bd8=(a=>{const b=d(a).versions;return h(b)});c.wbg.__wbg_node_caaf83d002149bd5=(a=>{const b=d(a).node;return h(b)});c.wbg.__wbg_msCrypto_0b84745e9245cdf6=(a=>{const b=d(a).msCrypto;return h(b)});c.wbg.__wbg_require_94a9da52636aacbf=function(){return H((()=>{const a=module.require;return h(a)}),arguments)};c.wbg.__wbg_randomFillSync_290977693942bf03=function(){return H(((a,b)=>{d(a).randomFillSync(g(b))}),arguments)};c.wbg.__wbg_getRandomValues_260cc23a41afad9a=function(){return H(((a,b)=>{d(a).getRandomValues(d(b))}),arguments)};c.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const c=d(a)==d(b);return c});c.wbg.__wbindgen_boolean_get=(a=>{const b=d(a);const c=typeof b===a1?(b?V:W):2;return c});c.wbg.__wbindgen_as_number=(a=>{const b=+d(a);return b});c.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const c=d(a)[d(b)];return h(c)});c.wbg.__wbg_set_f975102236d3c502=((a,b,c)=>{d(a)[g(b)]=g(c)});c.wbg.__wbg_error_a526fb08a0205972=((a,c)=>{var d=K(a,c).slice();b.__wbindgen_free(a,c*a6,a6);console.error(...d)});c.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=d(a).body;return o(b)?W:h(b)});c.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return H(((a,b,c)=>{const e=d(a).createElement(s(b,c));return h(e)}),arguments)};c.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return H(((a,b,c,e,f)=>{const g=d(a).createElementNS(b===W?T:s(b,c),s(e,f));return h(g)}),arguments)};c.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,c)=>{const e=d(a).createTextNode(s(b,c));return h(e)});c.wbg.__wbg_querySelector_a5f74efc5fa193dd=function(){return H(((a,b,c)=>{const e=d(a).querySelector(s(b,c));return o(e)?W:h(e)}),arguments)};c.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=d(a) instanceof Window}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_document_5100775d18896c16=(a=>{const b=d(a).document;return o(b)?W:h(b)});c.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=d(a).location;return h(b)});c.wbg.__wbg_history_bc4057de66a2015f=function(){return H((a=>{const b=d(a).history;return h(b)}),arguments)};c.wbg.__wbg_scrollX_911016e1eeb293ae=function(){return H((a=>{const b=d(a).scrollX;return b}),arguments)};c.wbg.__wbg_scrollY_fb49613f84149dbb=function(){return H((a=>{const b=d(a).scrollY;return b}),arguments)};c.wbg.__wbg_localStorage_e381d34d0c40c761=function(){return H((a=>{const b=d(a).localStorage;return o(b)?W:h(b)}),arguments)};c.wbg.__wbg_fetch_c4b6afebdb1f918e=((a,b)=>{const c=d(a).fetch(d(b));return h(c)});c.wbg.__wbg_instanceof_Element_6945fc210db80ea9=(a=>{let b;try{b=d(a) instanceof Element}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_namespaceURI_5235ee79fd5f6781=((a,c)=>{const e=d(c).namespaceURI;var f=o(e)?W:n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);var g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,c)=>{d(a).innerHTML=s(b,c)});c.wbg.__wbg_outerHTML_e073aa84e7bc1eaf=((a,c)=>{const e=d(c).outerHTML;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return H(((a,b,c)=>{d(a).removeAttribute(s(b,c))}),arguments)};c.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return H(((a,b,c,e,f)=>{d(a).setAttribute(s(b,c),s(e,f))}),arguments)};c.wbg.__wbg_state_9cc3f933b7d50acb=function(){return H((a=>{const b=d(a).state;return h(b)}),arguments)};c.wbg.__wbg_pushState_b8e8d346f8bb33fd=function(){return H(((a,b,c,e,f,g)=>{d(a).pushState(d(b),s(c,e),f===W?T:s(f,g))}),arguments)};c.wbg.__wbg_setchecked_931ff2ed2cd3ebfd=((a,b)=>{d(a).checked=b!==W});c.wbg.__wbg_value_47fe6384562f52ab=((a,c)=>{const e=d(c).value;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_setvalue_78cb4f1fef58ae98=((a,b,c)=>{d(a).value=s(b,c)});c.wbg.__wbg_instanceof_HtmlSelectElement_f0e1b0ac7b371ac0=(a=>{let b;try{b=d(a) instanceof HTMLSelectElement}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_value_47c64189244491be=((a,c)=>{const e=d(c).value;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_href_706b235ecfe6848c=function(){return H(((a,c)=>{const e=d(c).href;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f}),arguments)};c.wbg.__wbg_protocol_b7292c581cfe1e5c=function(){return H(((a,c)=>{const e=d(c).protocol;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f}),arguments)};c.wbg.__wbg_host_8f1b8ead257c8135=function(){return H(((a,c)=>{const e=d(c).host;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f}),arguments)};c.wbg.__wbg_pathname_5449afe3829f96a1=function(){return H(((a,c)=>{const e=d(c).pathname;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f}),arguments)};c.wbg.__wbg_search_489f12953342ec1f=function(){return H(((a,c)=>{const e=d(c).search;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f}),arguments)};c.wbg.__wbg_hash_553098e838e06c1d=function(){return H(((a,c)=>{const e=d(c).hash;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f}),arguments)};c.wbg.__wbg_reload_e61411ac20446229=function(){return H((a=>{d(a).reload()}),arguments)};c.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=d(a).parentNode;return o(b)?W:h(b)});c.wbg.__wbg_parentElement_347524db59fc2976=(a=>{const b=d(a).parentElement;return o(b)?W:h(b)});c.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=d(a).childNodes;return h(b)});c.wbg.__wbg_lastChild_83efe6d5da370e1f=(a=>{const b=d(a).lastChild;return o(b)?W:h(b)});c.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=d(a).nextSibling;return o(b)?W:h(b)});c.wbg.__wbg_setnodeValue_94b86af0cda24b90=((a,b,c)=>{d(a).nodeValue=b===W?T:s(b,c)});c.wbg.__wbg_textContent_8e392d624539e731=((a,c)=>{const e=d(c).textContent;var f=o(e)?W:n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);var g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return H((a=>{const b=d(a).cloneNode();return h(b)}),arguments)};c.wbg.__wbg_insertBefore_d2a001abf538c1f8=function(){return H(((a,b,c)=>{const e=d(a).insertBefore(d(b),d(c));return h(e)}),arguments)};c.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return H(((a,b)=>{const c=d(a).removeChild(d(b));return h(c)}),arguments)};c.wbg.__wbg_href_7bfb3b2fdc0a6c3f=((a,c)=>{const e=d(c).href;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_pathname_c5fe403ef9525ec6=((a,c)=>{const e=d(c).pathname;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_search_c68f506c44be6d1e=((a,c)=>{const e=d(c).search;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_setsearch_fd62f4de409a2bb3=((a,b,c)=>{d(a).search=s(b,c)});c.wbg.__wbg_hash_cdea7a9b7e684a42=((a,c)=>{const e=d(c).hash;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_sethash_9bacb48849d0016e=((a,b,c)=>{d(a).hash=s(b,c)});c.wbg.__wbg_new_67853c351755d2cf=function(){return H(((a,b)=>{const c=new URL(s(a,b));return h(c)}),arguments)};c.wbg.__wbg_newwithbase_6aabbfb1b2e6a1cb=function(){return H(((a,b,c,d)=>{const e=new URL(s(a,b),s(c,d));return h(e)}),arguments)};c.wbg.__wbg_newwithsrc_ca8c671638a5c104=function(){return H(((a,b)=>{const c=new Audio(s(a,b));return h(c)}),arguments)};c.wbg.__wbg_value_d7f5bfbd9302c14b=((a,c)=>{const e=d(c).value;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_setvalue_090972231f0a4f6f=((a,b,c)=>{d(a).value=s(b,c)});c.wbg.__wbg_getItem_164e8e5265095b87=function(){return H(((a,c,e,f)=>{const g=d(c).getItem(s(e,f));var h=o(g)?W:n(g,b.__wbindgen_malloc,b.__wbindgen_realloc);var j=i;q()[a/a6+ V]=j;q()[a/a6+ W]=h}),arguments)};c.wbg.__wbg_setItem_ba2bb41d73dac079=function(){return H(((a,b,c,e,f)=>{d(a).setItem(s(b,c),s(e,f))}),arguments)};c.wbg.__wbg_play_14d1cdf5b2b09482=function(){return H((a=>{const b=d(a).play();return h(b)}),arguments)};c.wbg.__wbg_instanceof_Blob_83ad3dd4c9c406f0=(a=>{let b;try{b=d(a) instanceof Blob}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_newwithu8arraysequenceandoptions_366f462e1b363808=function(){return H(((a,b)=>{const c=new Blob(d(a),d(b));return h(c)}),arguments)};c.wbg.__wbg_arrayBuffer_307ddd1bd1d04e23=(a=>{const b=d(a).arrayBuffer();return h(b)});c.wbg.__wbg_debug_7d879afce6cf56cb=((a,b,c,e)=>{console.debug(d(a),d(b),d(c),d(e))});c.wbg.__wbg_error_696630710900ec44=((a,b,c,e)=>{console.error(d(a),d(b),d(c),d(e))});c.wbg.__wbg_info_80803d9a3f0aad16=((a,b,c,e)=>{console.info(d(a),d(b),d(c),d(e))});c.wbg.__wbg_log_5bb5f88f245d7762=(a=>{console.log(d(a))});c.wbg.__wbg_log_151eb4333ef0fe39=((a,b,c,e)=>{console.log(d(a),d(b),d(c),d(e))});c.wbg.__wbg_warn_5d3f783b0bae8943=((a,b,c,e)=>{console.warn(d(a),d(b),d(c),d(e))});c.wbg.__wbg_newwithstrandinit_3fd6fba4083ff2d0=function(){return H(((a,b,c)=>{const e=new Request(s(a,b),d(c));return h(e)}),arguments)};c.wbg.__wbg_instanceof_Response_849eb93e75734b6e=(a=>{let b;try{b=d(a) instanceof Response}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_ok_38d7c30bbc66719e=(a=>{const b=d(a).ok;return b});c.wbg.__wbg_json_1d5f113e916d8675=function(){return H((a=>{const b=d(a).json();return h(b)}),arguments)};c.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=d(a) instanceof ShadowRoot}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=d(a).host;return h(b)});c.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=d(a).target;return o(b)?W:h(b)});c.wbg.__wbg_bubbles_abce839854481bc6=(a=>{const b=d(a).bubbles;return b});c.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=d(a).cancelBubble;return b});c.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=d(a).composedPath();return h(b)});c.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{d(a).preventDefault()});c.wbg.__wbg_href_2edbae9e92cdfeff=((a,c)=>{const e=d(c).href;const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbg_clientX_fef6bf7a6bcf41b8=(a=>{const b=d(a).clientX;return b});c.wbg.__wbg_clientY_df42f8fceab3cef2=(a=>{const b=d(a).clientY;return b});c.wbg.__wbg_ctrlKey_008695ce60a588f5=(a=>{const b=d(a).ctrlKey;return b});c.wbg.__wbg_shiftKey_1e76dbfcdd36a4b4=(a=>{const b=d(a).shiftKey;return b});c.wbg.__wbg_altKey_07da841b54bd3ed6=(a=>{const b=d(a).altKey;return b});c.wbg.__wbg_metaKey_86bfd3b0d3a8083f=(a=>{const b=d(a).metaKey;return b});c.wbg.__wbg_addEventListener_53b787075bd5e003=function(){return H(((a,b,c,e)=>{d(a).addEventListener(s(b,c),d(e))}),arguments)};c.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return H(((a,b,c,e,f)=>{d(a).addEventListener(s(b,c),d(e),d(f))}),arguments)};c.wbg.__wbg_removeEventListener_92cb9b3943463338=function(){return H(((a,b,c,e)=>{d(a).removeEventListener(s(b,c),d(e))}),arguments)};c.wbg.__wbg_removeEventListener_5d31483804421bfa=function(){return H(((a,b,c,e,f)=>{d(a).removeEventListener(s(b,c),d(e),f!==W)}),arguments)};c.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const c=d(a)[b>>>W];return h(c)});c.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=d(a).length;return b});c.wbg.__wbg_new_16b304a2cfa7ff4a=(()=>{const a=new R();return h(a)});c.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{const c=new Function(s(a,b));return h(c)});c.wbg.__wbg_call_27c0f87801dedf93=function(){return H(((a,b)=>{const c=d(a).call(d(b));return h(c)}),arguments)};c.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a7();return h(a)});c.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return H((()=>{const a=self.self;return h(a)}),arguments)};c.wbg.__wbg_window_c6fb939a7f436783=function(){return H((()=>{const a=window.window;return h(a)}),arguments)};c.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return H((()=>{const a=globalThis.globalThis;return h(a)}),arguments)};c.wbg.__wbg_global_207b558942527489=function(){return H((()=>{const a=global.global;return h(a)}),arguments)};c.wbg.__wbg_from_89e3fc3ba5e6fb48=(a=>{const b=R.from(d(a));return h(b)});c.wbg.__wbg_of_4a2b313a453ec059=(a=>{const b=R.of(d(a));return h(b)});c.wbg.__wbg_push_a5b05aedc7234f9f=((a,b)=>{const c=d(a).push(d(b));return c});c.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2=(a=>{let b;try{b=d(a) instanceof ArrayBuffer}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_instanceof_Error_e20bb56fd5591a93=(a=>{let b;try{b=d(a) instanceof _}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_message_5bf28016c2b49cfb=(a=>{const b=d(a).message;return h(b)});c.wbg.__wbg_name_e7429f0dda6079e2=(a=>{const b=d(a).name;return h(b)});c.wbg.__wbg_toString_ffe4c9ea3b3532e9=(a=>{const b=d(a).toString();return h(b)});c.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return H(((a,b,c)=>{const e=d(a).call(d(b),d(c));return h(e)}),arguments)};c.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2=(a=>{const b=Number.isSafeInteger(d(a));return b});c.wbg.__wbg_getTime_2bc4375165f02d15=(a=>{const b=d(a).getTime();return b});c.wbg.__wbg_new0_7d84e5b2cd9fdc73=(()=>{const a=new Date();return h(a)});c.wbg.__wbg_entries_95cc2c823b285a09=(a=>{const b=a7.entries(d(a));return h(b)});c.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const c=a7.is(d(a),d(b));return c});c.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(d(a));return h(b)});c.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const c=d(a).then(d(b));return h(c)});c.wbg.__wbg_then_a73caa9a87991566=((a,b,c)=>{const e=d(a).then(d(b),d(c));return h(e)});c.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=d(a).buffer;return h(b)});c.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,c)=>{const e=new X(d(a),b>>>W,c>>>W);return h(e)});c.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new X(d(a));return h(b)});c.wbg.__wbg_set_a47bac70306a19a7=((a,b,c)=>{d(a).set(d(b),c>>>W)});c.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=d(a).length;return b});c.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6=(a=>{let b;try{b=d(a) instanceof X}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new X(a>>>W);return h(b)});c.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,c)=>{const e=d(a).subarray(b>>>W,c>>>W);return h(e)});c.wbg.__wbg_stringify_8887fe74e1c50d81=function(){return H((a=>{const b=a4(d(a));return h(b)}),arguments)};c.wbg.__wbg_set_1f9b04f170055d33=function(){return H(((a,b,c)=>{const e=Reflect.set(d(a),d(b),d(c));return e}),arguments)};c.wbg.__wbindgen_debug_string=((a,c)=>{const e=v(d(c));const f=n(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=i;q()[a/a6+ V]=g;q()[a/a6+ W]=f});c.wbg.__wbindgen_throw=((a,b)=>{throw new _(s(a,b))});c.wbg.__wbindgen_memory=(()=>{const a=b.memory;return h(a)});c.wbg.__wbindgen_closure_wrapper2615=((a,b,c)=>{const d=x(a,b,a8,y);return h(d)});c.wbg.__wbindgen_closure_wrapper2617=((a,b,c)=>{const d=x(a,b,a8,z);return h(d)});c.wbg.__wbindgen_closure_wrapper3048=((a,b,c)=>{const d=x(a,b,1880,A);return h(d)});c.wbg.__wbindgen_closure_wrapper5253=((a,b,c)=>{const d=B(a,b,2963,E);return h(d)});c.wbg.__wbindgen_closure_wrapper5648=((a,b,c)=>{const d=x(a,b,3080,F);return h(d)});c.wbg.__wbindgen_closure_wrapper5704=((a,b,c)=>{const d=x(a,b,3106,G);return h(d)});return c});var x=((a,c,d,e)=>{const f={a:a,b:c,cnt:V,dtor:d};const g=(...a)=>{f.cnt++;const c=f.a;f.a=W;try{return e(c,f.b,...a)}finally{if(--f.cnt===W){b.__wbindgen_export_2.get(f.dtor)(c,f.b);w.unregister(f)}else{f.a=c}}};g.original=f;w.register(g,f,f);return g});var B=((a,c,d,e)=>{const f={a:a,b:c,cnt:V,dtor:d};const g=(...a)=>{f.cnt++;try{return e(f.a,f.b,...a)}finally{if(--f.cnt===W){b.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=W;w.unregister(f)}}};g.original=f;w.register(g,f,f);return g});var Q=(async(a)=>{if(b!==T)return b;if(typeof a===Y){a=new URL(`konnektoren-pwa_bg.wasm`,import.meta.url)};const c=M();if(typeof a===a2||typeof Request===$&&a instanceof Request||typeof URL===$&&a instanceof URL){a=fetch(a)};N(c);const {instance:d,module:e}=await L(await a,c);return O(d,e)});var K=((a,b)=>{a=a>>>W;const c=J();const d=c.subarray(a/a6,a/a6+ b);const e=[];for(let a=W;a<d.length;a++){e.push(g(d[a]))};return e});var q=(()=>{if(p===U||p.byteLength===W){p=new Int32Array(b.memory.buffer)};return p});import{connectSolanaWallet as a}from"./snippets/konnektoren-20c46b89d48a90b5/inline0.js";let b;const c=new R(S).fill(T);c.push(T,U,!0,!1);let e=c.length;let i=W;let j=U;const l=typeof TextEncoder!==Y?new TextEncoder(Z):{encode:()=>{throw _(`TextEncoder not available`)}};const m=typeof l.encodeInto===$?((a,b)=>l.encodeInto(a,b)):((a,b)=>{const c=l.encode(a);b.set(c);return {read:a.length,written:c.length}});let p=U;const r=typeof TextDecoder!==Y?new TextDecoder(Z,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw _(`TextDecoder not available`)}};if(typeof TextDecoder!==Y){r.decode()};let t=U;const w=typeof a5===Y?{register:()=>{},unregister:()=>{}}:new a5(a=>{b.__wbindgen_export_2.get(a.dtor)(a.a,a.b)});let C=S;let I=U;export default Q;export{P as initSync}
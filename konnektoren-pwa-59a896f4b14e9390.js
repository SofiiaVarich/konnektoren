let X=0,a3=4,$=`boolean`,T=1,S=null,a0=`string`,a1=`Object`,U=`undefined`,V=`utf-8`,Q=128,Z=`function`,_=`number`,P=Array,W=Error,a2=FinalizationRegistry,a4=Object,Y=Uint8Array,R=undefined;var g=(a=>{if(d===b.length)b.push(b.length+ T);const c=d;d=b[c];b[c]=a;return c});var N=(b=>{if(a!==R)return a;const c=K();L(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return M(d,b)});var M=((b,c)=>{a=b.exports;O.__wbindgen_wasm_module=c;m=S;o=S;G=S;i=S;a.__wbindgen_start();return a});var n=(()=>{if(m===S||m.byteLength===X){m=new Float64Array(a.memory.buffer)};return m});var C=((c,d,e)=>{try{a._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2f81d25a889e20c0(c,d,B(e))}finally{b[A++]=R}});var E=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h63e7588ee214efa0(c,d,B(e))}finally{b[A++]=R}});var f=(a=>{const b=c(a);e(a);return b});var x=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h97352cba416ea881(b,c)});var L=((a,b)=>{});var t=((a,b,c)=>{if(c===R){const c=r.encode(a);const d=b(c.length,T)>>>X;j().subarray(d,d+ c.length).set(c);q=c.length;return d};let d=a.length;let e=b(d,T)>>>X;const f=j();let g=X;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==X){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,T)>>>X;const b=j().subarray(e+ g,e+ d);const f=s(a,b);g+=f.written;e=c(e,d,g,T)>>>X};q=g;return e});var z=((b,c,d,e)=>{const f={a:b,b:c,cnt:T,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===X){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=X;v.unregister(f)}}};g.original=f;v.register(g,f,f);return g});function F(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var u=(a=>{const b=typeof a;if(b==_||b==$||a==S){return `${a}`};if(b==a0){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==S){return `Symbol`}else{return `Symbol(${b})`}};if(b==Z){const b=a.name;if(typeof b==a0&&b.length>X){return `Function(${b})`}else{return `Function`}};if(P.isArray(a)){const b=a.length;let c=`[`;if(b>X){c+=u(a[X])};for(let d=T;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>T){d=c[T]}else{return toString.call(a)};if(d==a1){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return a1}};if(a instanceof W){return `${a.name}: ${a.message}\n${a.stack}`};return d});var p=(()=>{if(o===S||o.byteLength===X){o=new Int32Array(a.memory.buffer)};return o});var B=(a=>{if(A==T)throw new W(`out of js stack`);b[--A]=a;return A});var c=(a=>b[a]);var e=(a=>{if(a<132)return;b[a]=d;d=a});var j=(()=>{if(i===S||i.byteLength===X){i=new Y(a.memory.buffer)};return i});var l=(a=>a===R||a===S);var H=(()=>{if(G===S||G.byteLength===X){G=new Uint32Array(a.memory.buffer)};return G});var k=((a,b)=>{a=a>>>X;return h.decode(j().subarray(a,a+ b))});var I=((a,b)=>{a=a>>>X;const c=H();const d=c.subarray(a/a3,a/a3+ b);const e=[];for(let a=X;a<d.length;a++){e.push(f(d[a]))};return e});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:T,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=X;try{return e(c,f.b,...b)}finally{if(--f.cnt===X){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var D=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h62f7c0bfce252bab(b,c,g(d))});var y=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h0a7d56c1ab9aad97(b,c,g(d))});var J=(async(a,b)=>{if(typeof Response===Z&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===Z){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var K=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==T){b.a=X;return !0};const c=!1;return c});b.wbg.__wbg_instanceof_ClipboardItem_f68dca2b15f12ca4=(a=>{let b;try{b=c(a) instanceof ClipboardItem}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_Window_6faa18fbaad60885=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_read_f3915f795a505cff=(a=>{const b=c(a).read();return g(b)});b.wbg.__wbg_readText_57eee82b45ff69bf=(a=>{const b=c(a).readText();return g(b)});b.wbg.__wbg_write_ea7f24b79f380473=((a,b)=>{const d=c(a).write(c(b));return g(d)});b.wbg.__wbg_writeText_168aa980f3ae6365=((a,b,d)=>{const e=c(a).writeText(k(b,d));return g(e)});b.wbg.__wbg_clipboard_c45a65d94018d435=(a=>{const b=c(a).clipboard;return l(b)?X:g(b)});b.wbg.__wbg_navigator_5e4972bacd50a0cb=(a=>{const b=c(a).navigator;return g(b)});b.wbg.__wbg_new_859e9d4be18d3aaa=function(){return F((a=>{const b=new ClipboardItem(c(a));return g(b)}),arguments)};b.wbg.__wbg_types_dfe555355d13be1e=(a=>{const b=c(a).types;return g(b)});b.wbg.__wbg_getType_d8891fee06a3d3fd=((a,b,d)=>{const e=c(a).getType(k(b,d));return g(e)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===Z;return b});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===a0;return b});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===_?d:R;n()[a/8+ T]=l(e)?X:e;p()[a/a3+ X]=!l(e)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===a0?e:R;var g=l(f)?X:t(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=q;p()[b/a3+ T]=h;p()[b/a3+ X]=g});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;p()[a/a3+ T]=l(d)?X:d;p()[a/a3+ X]=!l(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>X});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;p()[a/a3+ T]=l(d)?X:d;p()[a/a3+ X]=!l(d)});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>X});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>X});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;p()[a/a3+ T]=l(d)?X:d;p()[a/a3+ X]=!l(d)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new W();return g(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(k(b,c))}finally{a.__wbindgen_free(d,e,T)}});b.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return F(((a,b)=>{const d=setTimeout(c(a),b);return g(d)}),arguments)};b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return g(b)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==S;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===R;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new W(k(a,b));return g(c)});b.wbg.__wbg_crypto_d05b68a3572bb8ca=(a=>{const b=c(a).crypto;return g(b)});b.wbg.__wbg_process_b02b3570280d0366=(a=>{const b=c(a).process;return g(b)});b.wbg.__wbg_versions_c1cb42213cedf0f5=(a=>{const b=c(a).versions;return g(b)});b.wbg.__wbg_node_43b1089f407e4ec2=(a=>{const b=c(a).node;return g(b)});b.wbg.__wbg_msCrypto_10fc94afee92bd76=(a=>{const b=c(a).msCrypto;return g(b)});b.wbg.__wbg_require_9a7e0f667ead4995=function(){return F((()=>{const a=module.require;return g(a)}),arguments)};b.wbg.__wbg_randomFillSync_b70ccbdf4926a99d=function(){return F(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d=function(){return F(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===$?(b?T:X):2;return d});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return g(b)});b.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const d=c(a)[c(b)];return g(d)});b.wbg.__wbg_set_f975102236d3c502=((a,b,d)=>{c(a)[f(b)]=f(d)});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=I(b,c).slice();a.__wbindgen_free(b,c*a3,a3);console.error(...d)});b.wbg.__wbg_warn_71afa7f8150659a1=((b,c)=>{var d=I(b,c).slice();a.__wbindgen_free(b,c*a3,a3);console.warn(...d)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return l(b)?X:g(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return F(((a,b,d)=>{const e=c(a).createElement(k(b,d));return g(e)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return F(((a,b,d,e,f)=>{const h=c(a).createElementNS(b===X?R:k(b,d),k(e,f));return g(h)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{const e=c(a).createTextNode(k(b,d));return g(e)});b.wbg.__wbg_querySelector_a5f74efc5fa193dd=function(){return F(((a,b,d)=>{const e=c(a).querySelector(k(b,d));return l(e)?X:g(e)}),arguments)};b.wbg.__wbg_instanceof_Element_6945fc210db80ea9=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_5235ee79fd5f6781=((b,d)=>{const e=c(d).namespaceURI;var f=l(e)?X:t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{c(a).innerHTML=k(b,d)});b.wbg.__wbg_outerHTML_e073aa84e7bc1eaf=((b,d)=>{const e=c(d).outerHTML;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return F(((a,b,d)=>{c(a).removeAttribute(k(b,d))}),arguments)};b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return F(((a,b,d,e,f)=>{c(a).setAttribute(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return l(b)?X:g(b)});b.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_history_bc4057de66a2015f=function(){return F((a=>{const b=c(a).history;return g(b)}),arguments)};b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return F(((a,b,d,e,f)=>{c(a).addEventListener(k(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_5d31483804421bfa=function(){return F(((a,b,d,e,f)=>{c(a).removeEventListener(k(b,d),c(e),f!==X)}),arguments)};b.wbg.__wbg_state_9cc3f933b7d50acb=function(){return F((a=>{const b=c(a).state;return g(b)}),arguments)};b.wbg.__wbg_pushState_b8e8d346f8bb33fd=function(){return F(((a,b,d,e,f,g)=>{c(a).pushState(c(b),k(d,e),f===X?R:k(f,g))}),arguments)};b.wbg.__wbg_href_2edbae9e92cdfeff=((b,d)=>{const e=c(d).href;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return g(b)});b.wbg.__wbg_instanceof_Blob_83ad3dd4c9c406f0=(a=>{let b;try{b=c(a) instanceof Blob}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithu8arraysequenceandoptions_366f462e1b363808=function(){return F(((a,b)=>{const d=new Blob(c(a),c(b));return g(d)}),arguments)};b.wbg.__wbg_arrayBuffer_307ddd1bd1d04e23=(a=>{const b=c(a).arrayBuffer();return g(b)});b.wbg.__wbg_setchecked_931ff2ed2cd3ebfd=((a,b)=>{c(a).checked=b!==X});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_setvalue_78cb4f1fef58ae98=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=c(a).target;return l(b)?X:g(b)});b.wbg.__wbg_bubbles_abce839854481bc6=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return g(b)});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_href_706b235ecfe6848c=function(){return F(((b,d)=>{const e=c(d).href;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f}),arguments)};b.wbg.__wbg_protocol_b7292c581cfe1e5c=function(){return F(((b,d)=>{const e=c(d).protocol;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f}),arguments)};b.wbg.__wbg_host_8f1b8ead257c8135=function(){return F(((b,d)=>{const e=c(d).host;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f}),arguments)};b.wbg.__wbg_pathname_5449afe3829f96a1=function(){return F(((b,d)=>{const e=c(d).pathname;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f}),arguments)};b.wbg.__wbg_search_489f12953342ec1f=function(){return F(((b,d)=>{const e=c(d).search;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f}),arguments)};b.wbg.__wbg_hash_553098e838e06c1d=function(){return F(((b,d)=>{const e=c(d).hash;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f}),arguments)};b.wbg.__wbg_play_14d1cdf5b2b09482=function(){return F((a=>{const b=c(a).play();return g(b)}),arguments)};b.wbg.__wbg_value_d7f5bfbd9302c14b=((b,d)=>{const e=c(d).value;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_setvalue_090972231f0a4f6f=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_newwithsrc_ca8c671638a5c104=function(){return F(((a,b)=>{const c=new Audio(k(a,b));return g(c)}),arguments)};b.wbg.__wbg_href_7bfb3b2fdc0a6c3f=((b,d)=>{const e=c(d).href;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_pathname_c5fe403ef9525ec6=((b,d)=>{const e=c(d).pathname;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_search_c68f506c44be6d1e=((b,d)=>{const e=c(d).search;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_setsearch_fd62f4de409a2bb3=((a,b,d)=>{c(a).search=k(b,d)});b.wbg.__wbg_hash_cdea7a9b7e684a42=((b,d)=>{const e=c(d).hash;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_sethash_9bacb48849d0016e=((a,b,d)=>{c(a).hash=k(b,d)});b.wbg.__wbg_new_67853c351755d2cf=function(){return F(((a,b)=>{const c=new URL(k(a,b));return g(c)}),arguments)};b.wbg.__wbg_newwithbase_6aabbfb1b2e6a1cb=function(){return F(((a,b,c,d)=>{const e=new URL(k(a,b),k(c,d));return g(e)}),arguments)};b.wbg.__wbg_ctrlKey_008695ce60a588f5=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_1e76dbfcdd36a4b4=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_07da841b54bd3ed6=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_86bfd3b0d3a8083f=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return l(b)?X:g(b)});b.wbg.__wbg_parentElement_347524db59fc2976=(a=>{const b=c(a).parentElement;return l(b)?X:g(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return g(b)});b.wbg.__wbg_lastChild_83efe6d5da370e1f=(a=>{const b=c(a).lastChild;return l(b)?X:g(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return l(b)?X:g(b)});b.wbg.__wbg_setnodeValue_94b86af0cda24b90=((a,b,d)=>{c(a).nodeValue=b===X?R:k(b,d)});b.wbg.__wbg_textContent_8e392d624539e731=((b,d)=>{const e=c(d).textContent;var f=l(e)?X:t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return F((a=>{const b=c(a).cloneNode();return g(b)}),arguments)};b.wbg.__wbg_insertBefore_d2a001abf538c1f8=function(){return F(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return F(((a,b)=>{const d=c(a).removeChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>X];return g(d)});b.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_new_16b304a2cfa7ff4a=(()=>{const a=new P();return g(a)});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{const c=new Function(k(a,b));return g(c)});b.wbg.__wbg_call_27c0f87801dedf93=function(){return F(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a4();return g(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return F((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return F((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return F((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return F((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbg_from_89e3fc3ba5e6fb48=(a=>{const b=P.from(c(a));return g(b)});b.wbg.__wbg_of_4a2b313a453ec059=(a=>{const b=P.of(c(a));return g(b)});b.wbg.__wbg_push_a5b05aedc7234f9f=((a,b)=>{const d=c(a).push(c(b));return d});b.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return F(((a,b,d)=>{const e=c(a).call(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_entries_95cc2c823b285a09=(a=>{const b=a4.entries(c(a));return g(b)});b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=a4.is(c(a),c(b));return d});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(c(a));return g(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return g(d)});b.wbg.__wbg_then_a73caa9a87991566=((a,b,d)=>{const e=c(a).then(c(b),c(d));return g(e)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const e=new Y(c(a),b>>>X,d>>>X);return g(e)});b.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new Y(c(a));return g(b)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>X)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6=(a=>{let b;try{b=c(a) instanceof Y}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new Y(a>>>X);return g(b)});b.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,d)=>{const e=c(a).subarray(b>>>X,d>>>X);return g(e)});b.wbg.__wbg_set_1f9b04f170055d33=function(){return F(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a3+ T]=g;p()[b/a3+ X]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new W(k(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return g(b)});b.wbg.__wbindgen_closure_wrapper2416=((a,b,c)=>{const d=w(a,b,1450,x);return g(d)});b.wbg.__wbindgen_closure_wrapper2891=((a,b,c)=>{const d=w(a,b,1581,y);return g(d)});b.wbg.__wbindgen_closure_wrapper6511=((a,b,c)=>{const d=z(a,b,3288,C);return g(d)});b.wbg.__wbindgen_closure_wrapper6894=((a,b,c)=>{const d=w(a,b,3402,D);return g(d)});b.wbg.__wbindgen_closure_wrapper6956=((a,b,c)=>{const d=w(a,b,3424,E);return g(d)});return b});var O=(async(b)=>{if(a!==R)return a;if(typeof b===U){b=new URL(`konnektoren-pwa_bg.wasm`,import.meta.url)};const c=K();if(typeof b===a0||typeof Request===Z&&b instanceof Request||typeof URL===Z&&b instanceof URL){b=fetch(b)};L(c);const {instance:d,module:e}=await J(await b,c);return M(d,e)});let a;const b=new P(Q).fill(R);b.push(R,S,!0,!1);let d=b.length;const h=typeof TextDecoder!==U?new TextDecoder(V,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw W(`TextDecoder not available`)}};if(typeof TextDecoder!==U){h.decode()};let i=S;let m=S;let o=S;let q=X;const r=typeof TextEncoder!==U?new TextEncoder(V):{encode:()=>{throw W(`TextEncoder not available`)}};const s=typeof r.encodeInto===Z?((a,b)=>r.encodeInto(a,b)):((a,b)=>{const c=r.encode(a);b.set(c);return {read:a.length,written:c.length}});const v=typeof a2===U?{register:()=>{},unregister:()=>{}}:new a2(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let A=Q;let G=S;export default O;export{N as initSync}
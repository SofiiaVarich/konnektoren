let b=`loading-container`,c=`loading-message`,d=`Loading...`,e=Math;function a(){let a;return {onStart:()=>{a=performance.now();document.getElementById(b).style.display=`block`;document.getElementById(c).textContent=d},onProgress:({current:a,total:b})=>{const f=document.getElementById(`loading-progress`);const g=document.getElementById(c);if(b){const c=e.round((a/b)*100);f.style.width=`${c}%`;g.textContent=`Loading... ${c}%`}else{g.textContent=d}},onComplete:()=>{const c=performance.now();const d=c- a;const f=e.max(200- d,0);setTimeout(()=>{console.debug(`Loading... done!`);document.getElementById(b).style.display=`none`},f)},onSuccess:a=>{},onFailure:a=>{console.warn(`Loading... failed!`,a)}}}export{a as default}
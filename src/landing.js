document.addEventListener('DOMContentLoaded', () => {
    const breakerBtn = document.getElementById('breaker-btn');
    const statusText = document.querySelector('.status-text');
    const codeBlock = document.querySelector('.code-block code');

    // The Stealth Mode Script
    const scriptContent = `(function(){const activeMsg="%c 🚀 STRRATUMM GOD-MODE ACTIVATED! 🚀 ";const activeStyle="background:#00ff88;color:black;font-weight:bold;font-size:20px;padding:15px;border-radius:8px;border:3px solid #000;display:block;";console.log(activeMsg,activeStyle);console.log("%c Your visibility state is now frozen. Detection is disabled. ","color:#00ff88;font-weight:bold;");const n=(o,p,v)=>{try{Object.defineProperty(o,p,{get:()=>v,set:()=>{},configurable:true});}catch(e){console.log("%c [SKIP] Protected: "+p,"color:#ffaa00;");}};n(document,'visibilityState','visible');n(document,'webkitVisibilityState','visible');n(document,'hidden',false);document.hasFocus=()=>true;const b=['blur','focus','focusin','focusout','visibilitychange','webkitvisibilitychange','mouseleave','mouseout','resize','pagehide','beforeunload'];const s=(e)=>{e.stopImmediatePropagation();e.stopPropagation();return false;};const o=EventTarget.prototype.addEventListener;EventTarget.prototype.addEventListener=function(t,l,e){if(b.includes(t.toLowerCase()))return;return o.call(this,t,l,e);};b.forEach(v=>{window.addEventListener(v,s,true);document.addEventListener(v,s,true);n(window,'on'+v,null);n(document,'on'+v,null);});n(MouseEvent.prototype,'screenX',500);n(MouseEvent.prototype,'screenY',500);})();`;

    // Display a truncated or minified version in the UI for aesthetics
    codeBlock.textContent = scriptContent.substring(0, 150) + "... [CORE LOGIC ENCRYPTED]";

    breakerBtn.addEventListener('click', async () => {
        try {
            await navigator.clipboard.writeText(scriptContent);
            
            breakerBtn.textContent = 'COPIED!';
            breakerBtn.style.background = '#00ff88';
            breakerBtn.style.color = '#000';
            statusText.textContent = 'Script copied to clipboard. Paste in F12 console.';
            
            setTimeout(() => {
                breakerBtn.textContent = 'ACTIVATE CLOUD BYPASS';
                breakerBtn.style.background = '#000';
                breakerBtn.style.color = '#00ff88';
                statusText.textContent = 'Ready for injection';
            }, 3000);
            
        } catch (err) {
            console.error('Failed to copy: ', err);
            statusText.textContent = 'Copy failed. Please select and copy manually.';
        }
    });

    // Smooth scroll for nav links
    document.querySelectorAll('nav a').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            if (targetId === '#') return;
            const targetElement = document.querySelector(targetId);
            if (targetElement) {
                targetElement.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
});

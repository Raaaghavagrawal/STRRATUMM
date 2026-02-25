const { invoke } = window.__TAURI__.core;

// DOM elements
const form = document.getElementById('ai-form');
const promptInput = document.getElementById('prompt-input');
const responseArea = document.getElementById('response-area');
const statusBar = document.getElementById('status-bar');
const statusText = statusBar.querySelector('.status-text');
const closeBtn = document.getElementById('close-btn');
const sendBtn = form.querySelector('.send-btn');
const stealthToggle = document.getElementById('stealth-toggle');

// Load on startup
window.addEventListener('DOMContentLoaded', () => {
  promptInput.focus();
  updateStatus('Ready to chat', 'success');
});

// Close button handler
closeBtn.addEventListener('click', async () => {
  try {
    await invoke('hide_overlay');
  } catch (error) {
    console.error('Failed to hide overlay:', error);
  }
});

// Form submission handler
form.addEventListener('submit', async (e) => {
  e.preventDefault();

  const prompt = promptInput.value.trim();
  const selectedModel = document.querySelector('input[name="ai-model"]:checked').value;

  if (!prompt) {
    updateStatus('⚠️ Please enter a question', 'error');
    promptInput.focus();
    return;
  }

  // Clear input
  promptInput.value = '';

  // Add user message to chat
  addMessage(prompt, 'user');

  // Update status and disable input
  updateStatus(`🤔 Thinking<span class="loading-dots"></span>`, 'loading');
  sendBtn.disabled = true;
  promptInput.disabled = true;

  // Add thinking placeholder
  const thinkingBubble = document.createElement('div');
  thinkingBubble.className = 'message ai';
  thinkingBubble.innerHTML = `
    <div class="typing-indicator">
      <div class="typing-dot"></div>
      <div class="typing-dot"></div>
      <div class="typing-dot"></div>
    </div>
  `;
  responseArea.appendChild(thinkingBubble);
  responseArea.scrollTo({ top: responseArea.scrollHeight, behavior: 'smooth' });

  try {
    // Disable click-through while processing
    await invoke('set_click_through', { enabled: false });

    // Call AI command
    const response = await invoke('send_to_ai', {
      prompt: prompt,
      model: selectedModel
    });

    // Remove thinking bubble
    thinkingBubble.remove();

    // Add AI response to chat
    addMessage(response, 'ai');

    // Update status
    updateStatus('✅ Response received', 'success');

    // Re-enable input
    sendBtn.disabled = false;
    promptInput.disabled = false;
    promptInput.focus();

  } catch (error) {
    console.error('AI request failed:', error);
    addMessage(`❌ Error: ${error}`, 'error');
    updateStatus('❌ Request failed', 'error');

    // Re-enable input
    sendBtn.disabled = false;
    promptInput.disabled = false;
    promptInput.focus();
  }
});

// Add message to response area
function addMessage(text, type) {
  // Remove welcome message if it exists
  const welcomeMsg = responseArea.querySelector('.welcome-message');
  if (welcomeMsg) {
    welcomeMsg.remove();
  }

  const messageDiv = document.createElement('div');
  messageDiv.className = `message ${type}`;

  // If it's AI response, parse markdown. If user, just text.
  if (type === 'ai') {
    // Use marked to parse markdown
    messageDiv.innerHTML = marked.parse(text);
  } else {
    messageDiv.textContent = text;
  }

  responseArea.appendChild(messageDiv);

  // Smooth scroll to bottom
  responseArea.scrollTo({
    top: responseArea.scrollHeight,
    behavior: 'smooth'
  });
}

// Update status bar
function updateStatus(message, type = '') {
  statusBar.className = 'status-bar';
  if (type) {
    statusBar.classList.add(type);
  }
  statusText.innerHTML = message;
}

// Handle window visibility changes
document.addEventListener('visibilitychange', () => {
  if (!document.hidden) {
    promptInput.focus();
  }
});

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
  // Escape key to hide overlay
  if (e.key === 'Escape') {
    invoke('hide_overlay').catch(console.error);
  }
});

// Auto-focus on input when clicking anywhere in the overlay
responseArea.addEventListener('click', () => {
  promptInput.focus();
});

// Stealth Mode Toggle & script copying
stealthToggle.addEventListener('click', async () => {
  const isEnabled = stealthToggle.classList.toggle('active');

  if (isEnabled) {
    stealthToggle.innerHTML = '<span class="pulse"></span> Stealth ON';

    // Create a special message with a copy button
    const stealthInfo = document.createElement('div');
    stealthInfo.className = 'message ai stealth-msg';
    stealthInfo.innerHTML = `
      <div style="background: rgba(16, 185, 129, 0.1); border: 1px solid rgba(16, 185, 129, 0.2); border-radius: 12px; padding: 15px; margin-bottom: 10px;">
        <h3 style="color: #10b981; margin-bottom: 8px; font-size: 14px; display: flex; align-items: center; gap: 8px;">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
          Stealth Bypass Active
        </h3>
        <p style="font-size: 13px; color: #cbd5e1; margin-bottom: 12px; line-height: 1.5;">
          The bypass script is now in your clipboard. Paste it into your browser's console (F12) on the exam page to freeze your visibility state.
        </p>
        <button class="copy-script-btn" id="manual-copy" style="background: #10b981; color: black; border: none; padding: 6px 12px; border-radius: 6px; font-size: 12px; font-weight: 600; cursor: pointer; transition: 0.2s;">
          Click to Copy Again
        </button>
      </div>
    `;
    responseArea.appendChild(stealthInfo);
    responseArea.scrollTo({ top: responseArea.scrollHeight, behavior: 'smooth' });

    const scriptContent = `(function(){const n=(o,p,v)=>{Object.defineProperty(o,p,{get:()=>v,set:()=>{},configurable:false});};n(document,'visibilityState','visible');n(document,'webkitVisibilityState','visible');n(document,'hidden',false);document.hasFocus=()=>true;const b=['blur','focus','focusin','focusout','visibilitychange','webkitvisibilitychange','mouseleave','mouseout','resize','pagehide','beforeunload'];const s=(e)=>{e.stopImmediatePropagation();e.stopPropagation();return false;};const o=EventTarget.prototype.addEventListener;EventTarget.prototype.addEventListener=function(t,l,e){if(b.includes(t.toLowerCase()))return;return o.call(this,t,l,e);};b.forEach(v=>{window.addEventListener(v,s,true);document.addEventListener(v,s,true);n(window,'on'+v,null);n(document,'on'+v,null);});n(MouseEvent.prototype,'screenX',500);n(MouseEvent.prototype,'screenY',500);console.log("%c STRRATUMM GOD-MODE ACTIVE ","background:#00ff88;color:black;font-weight:bold;");})();`;

    // Copy function
    const copyToClipboard = async () => {
      try {
        await navigator.clipboard.writeText(scriptContent);
        updateStatus('📋 Script copied to clipboard!', 'success');
      } catch (err) {
        updateStatus('❌ Copy failed', 'error');
      }
    };

    // Initial copy
    await copyToClipboard();

    document.getElementById('manual-copy').onclick = copyToClipboard;
  } else {
    stealthToggle.innerHTML = '<span class="pulse"></span> Stealth OFF';
    updateStatus('Stealth Mode Deactivated', '');
  }
});

// Log when overlay is ready
console.log('✨ AI Overlay ready!');
console.log('📌 Press Ctrl+Space to toggle visibility');
console.log('🤖 Using Gemini 2.0 Flash (experimental)');

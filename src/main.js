const { invoke } = window.__TAURI__.core;

// DOM elements
const form = document.getElementById('ai-form');
const promptInput = document.getElementById('prompt-input');
const responseArea = document.getElementById('response-area');
const statusBar = document.getElementById('status-bar');
const statusText = statusBar.querySelector('.status-text');
const closeBtn = document.getElementById('close-btn');
const sendBtn = form.querySelector('.send-btn');

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

// Log when overlay is ready
console.log('✨ AI Overlay ready!');
console.log('📌 Press Ctrl+Space to toggle visibility');
console.log('🤖 Using Gemini 2.0 Flash (experimental)');

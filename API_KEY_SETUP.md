# 🔑 How to Add Your Gemini API Key

## Method 1: Using .env File (Recommended for Development)

1. **Create a `.env` file** in the project root:
   ```
   c:\Users\Ranvijay\OneDrive\Desktop\ai overlay\ai-overlay\.env
   ```

2. **Add your API key**:
   ```
   GEMINI_API_KEY=your_actual_api_key_here
   ```

3. **Install dotenv** (if not already):
   ```powershell
   npm install dotenv
   ```

4. **Update `package.json`** to load .env:
   ```json
   "scripts": {
     "tauri": "dotenv -e .env -- tauri"
   }
   ```

5. **Restart the app**:
   ```powershell
   npm run tauri dev
   ```

---

## Method 2: Using Windows Environment Variables (Recommended for Production)

### Option A: Set for Current Session
```powershell
$env:GEMINI_API_KEY = "your_actual_api_key_here"
npm run tauri dev
```

### Option B: Set Permanently (System-wide)
1. Open **System Properties** → **Environment Variables**
2. Click **New** under User variables
3. Variable name: `GEMINI_API_KEY`
4. Variable value: `your_actual_api_key_here`
5. Click **OK**
6. **Restart your terminal**
7. Run: `npm run tauri dev`

### Option C: Set via PowerShell (Permanent)
```powershell
[System.Environment]::SetEnvironmentVariable('GEMINI_API_KEY', 'your_actual_api_key_here', 'User')
```
Then restart your terminal.

---

## Method 3: Quick Test (Temporary)

Run this before starting the app:
```powershell
$env:GEMINI_API_KEY = "your_actual_api_key_here"
npm run tauri dev
```

---

## ✅ Verify It's Working

1. Press **Ctrl + Space** to show overlay
2. Make sure **Gemini** is selected
3. Type a question like "What is the capital of France?"
4. Press **Enter**

If you see a real response from Gemini, it's working! 🎉

---

## 🔒 Security Notes

- **Never commit** your API key to git
- The `.env` file is already in `.gitignore`
- Use environment variables for production
- Don't share your API key publicly

---

## 🐛 Troubleshooting

**Error: "API key not set"**
- Make sure you set the `GEMINI_API_KEY` environment variable
- Restart your terminal after setting it
- Check spelling: `GEMINI_API_KEY` (all caps)

**Error: "API error (403)"**
- Your API key might be invalid
- Get a new key from https://makersuite.google.com/app/apikey

**Error: "Request failed"**
- Check your internet connection
- Make sure the API key is correct

# OpenAI/OpenRouter API Usage Guide

This guide explains how to set up the environment variables required for ChatGPT integration in the MQTT client.

## Required Environment Variables

The Rust MQTT client requires environment variables to connect to ChatGPT/OpenRouter API:

1. **API Key** - Your authentication key (required)
2. **Base URL** (optional) - The API endpoint URL (defaults to OpenRouter)
3. **Model Name** (optional) - The AI model to use (defaults to free model: `x-ai/grok-4.1-fast`)

## Option 1: Using OpenRouter (Recommended for Testing)

OpenRouter provides access to multiple AI models and is often more cost-effective for testing.

### Step 1: Get Your OpenRouter API Key

1. Go to [OpenRouter.ai](https://openrouter.ai/)
2. Sign up or log in to your account
3. Navigate to **Keys** section in your dashboard
4. Create a new API key
5. Copy your API key (starts with `sk-or-v1-...`)

### Step 2: Set Environment Variables

**Linux/macOS:**
```bash
export OPENROUTER_API_KEY="sk-or-v1-your-actual-key-here"
# Base URL is optional (defaults to OpenRouter)
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
# Model is optional (defaults to free model: x-ai/grok-4.1-fast)
export OPENROUTER_MODEL="x-ai/grok-4.1-fast"
```

**Note:** The base URL and model have defaults, so you only need to set the API key for basic usage!

**Windows (PowerShell):**
```powershell
$env:OPENROUTER_API_KEY="sk-or-v1-your-actual-key-here"
# Optional - defaults to OpenRouter
$env:OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
$env:OPENROUTER_MODEL="x-ai/grok-4.1-fast"
```

**Windows (Command Prompt):**
```cmd
set OPENROUTER_API_KEY=sk-or-v1-your-actual-key-here
set OPENROUTER_BASE_URL=https://openrouter.ai/api/v1
set OPENROUTER_MODEL=x-ai/grok-4.1-fast
```

### Step 3: Verify Variables Are Set

**Linux/macOS:**
```bash
echo $OPENROUTER_API_KEY
echo $OPENROUTER_BASE_URL
```

**Windows (PowerShell):**
```powershell
echo $env:OPENROUTER_API_KEY
echo $env:OPENROUTER_BASE_URL
```

## Option 2: Using OpenAI Official API

If you prefer to use OpenAI's official API:

### Step 1: Get Your OpenAI API Key

1. Go to [platform.openai.com](https://platform.openai.com/)
2. Sign up or log in to your account
3. Navigate to **API Keys** section
4. Create a new secret key
5. Copy your API key (starts with `sk-...`)

### Step 2: Set Environment Variables

**Linux/macOS:**
```bash
export OPENAI_API_KEY="sk-your-actual-key-here"
# Base URL is optional, defaults to OpenRouter, override for OpenAI
export OPENAI_API_BASE="https://api.openai.com/v1"
# Model is optional (defaults to free OpenRouter model, override for OpenAI)
export OPENAI_MODEL="gpt-3.5-turbo"
```

**Windows (PowerShell):**
```powershell
$env:OPENAI_API_KEY="sk-your-actual-key-here"
$env:OPENAI_API_BASE="https://api.openai.com/v1"
```

**Windows (Command Prompt):**
```cmd
set OPENAI_API_KEY=sk-your-actual-key-here
set OPENAI_API_BASE=https://api.openai.com/v1
```

## Option 3: Using LM Studio (Local)

If you're running LM Studio locally for testing:

**Linux/macOS:**
```bash
export OPENAI_API_KEY="lm-studio"  # Can be any value, LM Studio doesn't check
export OPENAI_API_BASE="http://localhost:1234/v1"
```

**Windows:**
```powershell
$env:OPENAI_API_KEY="lm-studio"
$env:OPENAI_API_BASE="http://localhost:1234/v1"
```

## Making Environment Variables Persistent

### Linux/macOS

Add to your `~/.bashrc` or `~/.zshrc`:
```bash
export OPENROUTER_API_KEY="sk-or-v1-your-actual-key-here"
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
```

Then reload:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

### Windows

Add to System Environment Variables:
1. Open **System Properties** → **Environment Variables**
2. Under **User variables**, click **New**
3. Add `OPENROUTER_API_KEY` with your key value
4. Add `OPENROUTER_BASE_URL` with `https://openrouter.ai/api/v1`
5. Click **OK** to save

## Testing the Setup

Run the integration tests to verify your API key works:

```bash
cd computerb/mqtt_client
cargo test --test integration_test -- --nocapture
```

You should see:
```
✅ Success! ChatGPT response: [response text]
✅ Integration test passed!
```

## Running the MQTT Client

Once environment variables are set, run the client:

```bash
cd computerb/mqtt_client
cargo run --release
```

The client will:
1. Connect to MQTT broker
2. Subscribe to `/esp_gpt_out` topic
3. Wait for ChatGPT responses from ESP32
4. Call ChatGPT API when messages are received
5. Publish responses to `/client_gpt` topic

## Troubleshooting

### "API key environment variable must be set"

- Verify the environment variable is set: `echo $OPENROUTER_API_KEY`
- Make sure you're in the same terminal session where you set the variable
- Check for typos in the variable name

### "Failed to call OpenAI API"

- Verify your API key is valid and has credits/quota
- Check your internet connection
- Verify the base URL is correct
- For OpenRouter, ensure you've set `OPENROUTER_BASE_URL`

### "Connection refused" (LM Studio)

- Ensure LM Studio is running
- Check that LM Studio's API server is enabled
- Verify the port (default is 1234)
- Try `curl http://localhost:1234/v1/models` to test

## Security Notes

⚠️ **Important Security Tips:**

- **Never commit API keys to git** - They should only be in environment variables
- **Don't share your API keys** - Treat them like passwords
- **Use `.env` files carefully** - If using `.env`, ensure it's in `.gitignore`
- **Rotate keys regularly** - If a key is exposed, revoke it immediately

## Variable Priority and Defaults

The client checks environment variables in this order:

1. **API Key**: `OPENROUTER_API_KEY` → `OPENAI_API_KEY` (required, no default)
2. **Base URL**: `OPENROUTER_BASE_URL` → `OPENAI_API_BASE` (defaults to `https://openrouter.ai/api/v1`)
3. **Model**: `OPENROUTER_MODEL` → `OPENAI_MODEL` (defaults to `x-ai/grok-4.1-fast` - free model)

If both OpenRouter and OpenAI variables are set, OpenRouter variables take precedence.

### Default Configuration

By default, the client is configured for OpenRouter with a free model:
- **Endpoint**: `https://openrouter.ai/api/v1`
- **Model**: `x-ai/grok-4.1-fast` (free, no credits required)

You only need to set `OPENROUTER_API_KEY` to get started!

## Available Free Models on OpenRouter

OpenRouter offers several free models you can use:

- **`x-ai/grok-4.1-fast`** (default) - Fast and free
- **`google/gemini-flash-1.5`** - Google's free model
- **`meta-llama/llama-3.2-3b-instruct:free`** - Meta's free Llama model
- **`qwen/qwen-2.5-7b-instruct:free`** - Qwen free model

To use a different model:
```bash
export OPENROUTER_MODEL="google/gemini-flash-1.5"
```

Browse all available models at [OpenRouter Models](https://openrouter.ai/models)

## Example: Complete Setup Script

**Linux/macOS:**
```bash
#!/bin/bash
# setup_openai.sh

export OPENROUTER_API_KEY="sk-or-v1-your-actual-key-here"
# Optional - these are the defaults
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
export OPENROUTER_MODEL="x-ai/grok-4.1-fast"

echo "✅ Environment variables set!"
echo "API Key: ${OPENROUTER_API_KEY:0:10}..."
echo "Base URL: $OPENROUTER_BASE_URL"
echo "Model: $OPENROUTER_MODEL"

# Run the client
cd computerb/mqtt_client
cargo run --release
```

**Windows (PowerShell):**
```powershell
# setup_openai.ps1

$env:OPENROUTER_API_KEY="sk-or-v1-your-actual-key-here"
# Optional - these are the defaults
$env:OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"
$env:OPENROUTER_MODEL="x-ai/grok-4.1-fast"

Write-Host "✅ Environment variables set!"
Write-Host "API Key: $($env:OPENROUTER_API_KEY.Substring(0,10))..."
Write-Host "Base URL: $env:OPENROUTER_BASE_URL"
Write-Host "Model: $env:OPENROUTER_MODEL"

# Run the client
cd computerb/mqtt_client
cargo run --release
```

## Additional Resources

- [OpenRouter Documentation](https://openrouter.ai/docs)
- [OpenAI API Documentation](https://platform.openai.com/docs)
- [LM Studio Documentation](https://lmstudio.ai/docs)


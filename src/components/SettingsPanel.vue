<script setup lang="ts">
import { ref } from 'vue'
import { useSyncStore } from '../stores/sync'

const store = useSyncStore()
const isExpanded = ref(false)
const hotkeyInput = ref(store.hotkey)
const hotkeyError = ref('')
const isCapturing = ref(false)

const KEY_NAMES: Record<string, string> = {
  ' ': 'Space',
  'ArrowUp': 'Up',
  'ArrowDown': 'Down',
  'ArrowLeft': 'Left',
  'ArrowRight': 'Right',
  'Escape': 'Esc',
}

function formatKey(e: KeyboardEvent): string {
  const parts: string[] = []

  if (e.metaKey) parts.push('Cmd')
  if (e.ctrlKey) parts.push('Ctrl')
  if (e.altKey) parts.push('Alt')
  if (e.shiftKey) parts.push('Shift')

  const key = e.key
  if (!['Control', 'Alt', 'Shift', 'Meta'].includes(key)) {
    let formatted = KEY_NAMES[key] ?? key
    if (formatted.length === 1) {
      formatted = formatted.toUpperCase()
    }
    parts.push(formatted)
  }

  return parts.join('+')
}

async function handleKeyDown(e: KeyboardEvent) {
  e.preventDefault()
  e.stopPropagation()

  const key = e.key
  const isModifierOnly = ['Control', 'Alt', 'Shift', 'Meta'].includes(key)
  if (isModifierOnly) return

  const combo = formatKey(e)
  if (combo && combo !== store.hotkey) {
    hotkeyInput.value = combo
    hotkeyError.value = ''
    try {
      await store.setHotkey(combo)
    } catch (err) {
      hotkeyError.value = String(err)
      hotkeyInput.value = store.hotkey
    }
  }
  ;(e.target as HTMLInputElement).blur()
}

function handleBlur() {
  isCapturing.value = false
}

function handleAutoConnectChange() {
  store.persistSettings()
}
</script>

<template>
  <div class="border border-gray-200 dark:border-gray-700 rounded">
    <button
      @click="isExpanded = !isExpanded"
      class="w-full flex items-center justify-between px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800"
    >
      <span class="flex items-center gap-2">
        <span>⚙️</span>
        <span>Settings</span>
      </span>
      <span :class="['transition-transform', isExpanded && 'rotate-180']">▼</span>
    </button>

    <div v-if="isExpanded" class="border-t border-gray-200 dark:border-gray-700 p-3 space-y-4">
      <label class="flex items-center gap-2 text-sm">
        <input
          type="checkbox"
          v-model="store.autoConnect"
          @change="handleAutoConnectChange"
          class="rounded"
        />
        <span class="text-gray-700 dark:text-gray-300">Auto-connect on startup</span>
      </label>

      <div>
        <label class="block text-xs text-gray-600 dark:text-gray-400 mb-1">
          Global Hotkey (click and press keys)
        </label>
        <input
          :value="isCapturing ? 'Press keys...' : hotkeyInput"
          type="text"
          readonly
          placeholder="Click to set hotkey"
          @focus="isCapturing = true"
          @blur="handleBlur"
          @keydown="handleKeyDown"
          :class="[
            'w-full px-3 py-1.5 text-sm border rounded cursor-pointer',
            isCapturing
              ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
              : 'border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800'
          ]"
        />
        <p v-if="hotkeyError" class="text-xs text-red-500 mt-1">{{ hotkeyError }}</p>
        <p v-else-if="store.hotkey" class="text-xs text-green-600 mt-1">
          Current: {{ store.hotkey }}
        </p>
      </div>
    </div>
  </div>
</template>

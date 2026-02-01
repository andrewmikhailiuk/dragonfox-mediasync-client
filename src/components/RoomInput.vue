<script setup lang="ts">
import { useSyncStore } from '../stores/sync'

const store = useSyncStore()

function handleSubmit() {
  if (store.isConnected) {
    store.disconnect()
  } else {
    store.connect()
  }
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="space-y-3">
    <div>
      <label class="block text-xs text-gray-600 dark:text-gray-400 mb-1">Server URL</label>
      <input
        v-model="store.serverUrl"
        type="text"
        placeholder="ws://localhost:8080"
        :disabled="store.isConnected || store.isConnecting"
        class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 disabled:opacity-50"
      />
    </div>
    <div>
      <label class="block text-xs text-gray-600 dark:text-gray-400 mb-1">Room</label>
      <input
        v-model="store.room"
        type="text"
        placeholder="room name"
        :disabled="store.isConnected || store.isConnecting"
        class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 disabled:opacity-50"
      />
    </div>
    <button
      type="submit"
      :disabled="store.isConnecting"
      :class="[
        'w-full py-2 text-sm font-medium rounded transition-colors',
        store.isConnected
          ? 'bg-red-500 hover:bg-red-600 text-white'
          : 'bg-blue-500 hover:bg-blue-600 text-white',
        store.isConnecting && 'opacity-50 cursor-not-allowed'
      ]"
    >
      {{ store.isConnected ? 'Disconnect' : store.isConnecting ? 'Connecting...' : 'Connect' }}
    </button>
  </form>
</template>

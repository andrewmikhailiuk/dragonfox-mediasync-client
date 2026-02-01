<script setup lang="ts">
import { computed } from 'vue'
import { useSyncStore } from '../stores/sync'

const store = useSyncStore()

const statusText = computed(() => {
  const s = store.connectionStatus
  switch (s.status) {
    case 'disconnected':
      return 'Disconnected'
    case 'connecting':
      return 'Connecting...'
    case 'connected':
      return `Connected to ${s.room}`
    case 'reconnecting':
      return `Reconnecting (attempt ${s.attempt})...`
  }
})

const latencyText = computed(() => {
  const s = store.connectionStatus
  if (s.status === 'connected' && s.latencyMs !== undefined) {
    return `${s.latencyMs}ms`
  }
  return null
})

const statusColor = computed(() => {
  switch (store.connectionStatus.status) {
    case 'connected':
      return 'bg-green-500'
    case 'connecting':
    case 'reconnecting':
      return 'bg-yellow-500'
    default:
      return 'bg-gray-400'
  }
})
</script>

<template>
  <div class="flex items-center gap-2 text-sm">
    <span :class="['w-2 h-2 rounded-full', statusColor]"></span>
    <span class="text-gray-700 dark:text-gray-300">{{ statusText }}</span>
    <span v-if="latencyText" class="text-gray-500 text-xs">({{ latencyText }})</span>
  </div>
</template>

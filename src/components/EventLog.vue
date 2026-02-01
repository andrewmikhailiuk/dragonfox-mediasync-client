<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useSyncStore } from '../stores/sync'

const store = useSyncStore()
const logContainer = ref<HTMLElement | null>(null)

function formatTime(timestamp: number): string {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

function formatClientId(clientId?: string): string {
  if (!clientId) return ''
  return clientId.length > 8 ? clientId.slice(0, 8) : clientId
}

watch(
  () => store.eventLog.length,
  async () => {
    await nextTick()
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  }
)
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex items-center justify-between mb-2">
      <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Event Log</span>
      <span class="text-xs text-gray-500">{{ store.eventLog.length }} events</span>
    </div>
    <div
      ref="logContainer"
      class="flex-1 overflow-y-auto bg-gray-50 dark:bg-gray-900 rounded border border-gray-200 dark:border-gray-700 p-2 font-mono text-xs"
    >
      <div v-if="store.eventLog.length === 0" class="text-gray-400 text-center py-4">
        No events yet
      </div>
      <div
        v-for="(event, index) in store.eventLog"
        :key="index"
        :class="[
          'py-0.5',
          event.direction === 'in' ? 'text-blue-600 dark:text-blue-400' : 'text-green-600 dark:text-green-400'
        ]"
      >
        <span class="text-gray-500">[{{ formatTime(event.timestamp) }}]</span>
        <span class="mx-1">{{ event.direction === 'in' ? '←' : '→' }}</span>
        <span>{{ event.type }}</span>
        <span v-if="event.clientId" class="text-gray-500 ml-1">({{ formatClientId(event.clientId) }})</span>
      </div>
    </div>
  </div>
</template>

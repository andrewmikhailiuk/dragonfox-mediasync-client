<script setup lang="ts">
import { onMounted } from 'vue'
import { useSyncStore } from './stores/sync'
import ConnectionStatus from './components/ConnectionStatus.vue'
import RoomInput from './components/RoomInput.vue'
import EventLog from './components/EventLog.vue'
import SettingsPanel from './components/SettingsPanel.vue'

const store = useSyncStore()

onMounted(() => {
  store.init()
})
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100">
    <header class="p-4 border-b border-gray-200 dark:border-gray-700">
      <h1 class="text-lg font-semibold mb-2">Dragonfox Media Sync</h1>
      <ConnectionStatus />
    </header>

    <main class="flex-1 flex flex-col p-4 gap-4 overflow-hidden">
      <RoomInput />
      <SettingsPanel />
      <div class="flex-1 min-h-0">
        <EventLog />
      </div>
    </main>
  </div>
</template>

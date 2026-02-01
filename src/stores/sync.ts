import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export interface SyncEvent {
  type: string
  timestamp: number
  clientId?: string
  position?: number
  direction: 'in' | 'out'
}

export interface ConnectionStatus {
  status: 'disconnected' | 'connecting' | 'connected' | 'reconnecting'
  room?: string
  latencyMs?: number
  attempt?: number
}

const STORAGE_KEY = 'media-sync-settings'
const MAX_EVENTS = 50

interface Settings {
  serverUrl: string
  room: string
  autoConnect: boolean
  hotkey: string
}

function generateUUID(): string {
  return crypto.randomUUID()
}

function loadSettings(): Settings {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      const parsed = JSON.parse(saved)
      if (parsed.room) {
        return parsed
      }
    }
  } catch {}
  return {
    serverUrl: 'ws://localhost:8080',
    room: generateUUID(),
    autoConnect: false,
    hotkey: ''
  }
}

function saveSettings(settings: Settings) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
}

export const useSyncStore = defineStore('sync', () => {
  const settings = loadSettings()

  const serverUrl = ref(settings.serverUrl)
  const room = ref(settings.room)
  const autoConnect = ref(settings.autoConnect)
  const hotkey = ref(settings.hotkey)
  const connectionStatus = ref<ConnectionStatus>({ status: 'disconnected' })
  const eventLog = ref<SyncEvent[]>([])

  const isConnected = computed(() => connectionStatus.value.status === 'connected')
  const isConnecting = computed(() =>
    connectionStatus.value.status === 'connecting' ||
    connectionStatus.value.status === 'reconnecting'
  )

  function persistSettings() {
    saveSettings({
      serverUrl: serverUrl.value,
      room: room.value,
      autoConnect: autoConnect.value,
      hotkey: hotkey.value
    })
  }

  function addEvent(event: Omit<SyncEvent, 'direction'>, direction: 'in' | 'out') {
    eventLog.value.push({ ...event, direction })
    if (eventLog.value.length > MAX_EVENTS) {
      eventLog.value = eventLog.value.slice(-MAX_EVENTS)
    }
  }

  async function connect() {
    try {
      persistSettings()
      await invoke('connect', {
        serverUrl: serverUrl.value,
        room: room.value
      })
    } catch (e) {
      console.error('Failed to connect:', e)
    }
  }

  async function disconnect() {
    try {
      await invoke('disconnect')
    } catch (e) {
      console.error('Failed to disconnect:', e)
    }
  }

  async function sendToggle() {
    try {
      await invoke('send_toggle')
    } catch (e) {
      console.error('Failed to send toggle:', e)
    }
  }

  async function setHotkey(shortcut: string) {
    try {
      await invoke('set_hotkey', { shortcut })
      hotkey.value = shortcut
      persistSettings()
    } catch (e) {
      console.error('Failed to set hotkey:', e)
      throw e
    }
  }

  async function init() {
    // Listen for connection status changes
    await listen<ConnectionStatus>('connection-status', (event) => {
      connectionStatus.value = event.payload
    })

    // Listen for sync events
    await listen<{ type: string; timestamp: number; clientId?: string }>('sync-event', (event) => {
      const { type, timestamp, clientId } = event.payload
      const direction = clientId ? 'in' : 'out'
      addEvent({ type, timestamp, clientId }, direction)
    })

    // Set initial hotkey if configured
    if (hotkey.value) {
      try {
        await invoke('set_hotkey', { shortcut: hotkey.value })
      } catch (e) {
        console.error('Failed to restore hotkey:', e)
      }
    }

    // Auto-connect if enabled
    if (autoConnect.value) {
      await connect()
    }
  }

  return {
    serverUrl,
    room,
    autoConnect,
    hotkey,
    connectionStatus,
    eventLog,
    isConnected,
    isConnecting,
    connect,
    disconnect,
    sendToggle,
    setHotkey,
    init,
    persistSettings
  }
})

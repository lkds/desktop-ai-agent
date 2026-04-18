import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

interface ProviderConfig {
  kind: 'openai' | 'claude' | 'ollama' | 'custom'
  api_key?: string
  base_url?: string
  model: string
}

interface ConfigState {
  providers: ProviderConfig[]
  currentProvider: ProviderConfig | null
  currentModel: string | null
  allowedPaths: string[]
  
  // Actions
  loadConfig: () => Promise<void>
  saveConfig: () => Promise<void>
  setProvider: (provider: ProviderConfig) => void
  addAllowedPath: (path: string) => void
  removeAllowedPath: (path: string) => void
}

export const useConfigStore = create<ConfigState>((set, get) => ({
  providers: [],
  currentProvider: null,
  currentModel: null,
  allowedPaths: [],
  
  loadConfig: async () => {
    try {
      const config = await invoke<{
        providers: ProviderConfig[]
        current_provider: ProviderConfig
        allowed_paths: string[]
      }>('get_config')
      
      set({
        providers: config.providers,
        currentProvider: config.current_provider,
        currentModel: config.current_provider?.model,
        allowedPaths: config.allowed_paths
      })
    } catch (error) {
      console.error('Load config failed:', error)
    }
  },
  
  saveConfig: async () => {
    try {
      await invoke('save_config', {
        config: {
          providers: get().providers,
          current_provider: get().currentProvider,
          allowed_paths: get().allowedPaths
        }
      })
    } catch (error) {
      console.error('Save config failed:', error)
    }
  },
  
  setProvider: (provider) => {
    set({
      currentProvider: provider,
      currentModel: provider.model
    })
    get().saveConfig()
  },
  
  addAllowedPath: (path) => {
    set(state => ({
      allowedPaths: [...state.allowedPaths, path]
    }))
    get().saveConfig()
  },
  
  removeAllowedPath: (path) => {
    set(state => ({
      allowedPaths: state.allowedPaths.filter(p => p !== path)
    }))
    get().saveConfig()
  }
}))
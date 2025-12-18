// Configuration store using Zustand 5.0

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';

export interface ApiConfig {
  endpoint: string;
  api_key: string;
  model: string;
  supports_vision: boolean;
  max_tokens: number;
  temperature: number;
}

export interface AppConfig {
  api: ApiConfig;
  language: string;
  theme: string;
  global_shortcut: string;
}

interface ConfigState {
  config: AppConfig;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadConfig: () => Promise<void>;
  saveConfig: (config: AppConfig) => Promise<void>;
  updateApi: (api: Partial<ApiConfig>) => void;
  testConnection: () => Promise<string>;
}

const defaultConfig: AppConfig = {
  api: {
    endpoint: 'https://api.openai.com/v1',
    api_key: '',
    model: 'gpt-4o',
    supports_vision: true,
    max_tokens: 4096,
    temperature: 0.7,
  },
  language: 'ko',
  theme: 'dark',
  global_shortcut: 'Shift+Alt+A',
};

export const useConfigStore = create<ConfigState>()(
  persist(
    (set, get) => ({
      config: defaultConfig,
      isLoading: false,
      error: null,

      loadConfig: async () => {
        set({ isLoading: true, error: null });
        try {
          const config = await invoke<AppConfig>('get_config');
          set({ config, isLoading: false });
        } catch (e) {
          set({ error: String(e), isLoading: false });
        }
      },

      saveConfig: async (config: AppConfig) => {
        set({ isLoading: true, error: null });
        try {
          await invoke('save_config', { config });
          set({ config, isLoading: false });
        } catch (e) {
          set({ error: String(e), isLoading: false });
        }
      },

      updateApi: (api: Partial<ApiConfig>) => {
        const { config } = get();
        set({
          config: {
            ...config,
            api: { ...config.api, ...api },
          },
        });
      },

      testConnection: async () => {
        const { config } = get();
        try {
          const result = await invoke<string>('test_api_connection', { config: config.api });
          return result;
        } catch (e) {
          throw new Error(String(e));
        }
      },
    }),
    {
      name: 'automate-config',
      partialize: (state) => ({ config: state.config }),
    }
  )
);

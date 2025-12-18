// Guide store using Zustand 5.0

import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface GuideEntry {
  name: string;
  path: string;
  is_dir: boolean;
}

export interface GuideIndexEntry {
  path: string;
  title: string;
}

interface GuideState {
  guides: GuideIndexEntry[];
  isLoading: boolean;
  error: string | null;

  // Actions
  loadGuides: () => Promise<void>;
  createGuide: (userInput: string) => Promise<{ path: string; title: string }>;
  searchGuide: (query: string) => Promise<string>;
  readGuide: (path: string) => Promise<string>;
}

export const useGuideStore = create<GuideState>()((set) => ({
  guides: [],
  isLoading: false,
  error: null,

  loadGuides: async () => {
    set({ isLoading: true, error: null });
    try {
      const guides = await invoke<GuideIndexEntry[]>('guide_index');
      set({ guides, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  createGuide: async (userInput: string) => {
    set({ isLoading: true, error: null });
    try {
      const result = await invoke<{ path: string; title: string }>('guide_create', {
        request: { user_input: userInput },
      });
      // Reload guides after creation
      const guides = await invoke<GuideIndexEntry[]>('guide_index');
      set({ guides, isLoading: false });
      return result;
    } catch (e) {
      set({ error: String(e), isLoading: false });
      throw e;
    }
  },

  searchGuide: async (query: string) => {
    try {
      const result = await invoke<string>('guide_search', { query });
      return result;
    } catch (e) {
      throw new Error(String(e));
    }
  },

  readGuide: async (path: string) => {
    try {
      const content = await invoke<string>('guide_read', { path });
      return content;
    } catch (e) {
      throw new Error(String(e));
    }
  },
}));

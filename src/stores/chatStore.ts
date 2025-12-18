// Chat store using Zustand 5.0

import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
  action?: ActionResponse;
}

export interface ActionResponse {
  thought: string;
  action: string;
  params: Record<string, unknown>;
}

interface ChatState {
  messages: Message[];
  isProcessing: boolean;
  error: string | null;

  // Actions
  sendMessage: (content: string, includeScreen?: boolean) => Promise<void>;
  addMessage: (message: Omit<Message, 'id' | 'timestamp'>) => void;
  clearMessages: () => void;
  executeAction: (action: ActionResponse) => Promise<void>;
}

const generateId = () => Math.random().toString(36).substring(2, 9);

// Helper to update overlay status
const setOverlayStatus = async (status: string, message?: string) => {
  try {
    await invoke('overlay_status', { status, message });
  } catch (e) {
    console.warn('Failed to set overlay status:', e);
  }
};

// Helper to trigger click effect
const triggerClickEffect = async (x: number, y: number, button?: string) => {
  try {
    await invoke('overlay_click', { x, y, button });
  } catch (e) {
    console.warn('Failed to trigger click effect:', e);
  }
};

// Helper to update cursor position in overlay
const updateOverlayCursor = async (x: number, y: number) => {
  try {
    await invoke('overlay_cursor_move', { x, y });
  } catch (e) {
    console.warn('Failed to update cursor:', e);
  }
};

export const useChatStore = create<ChatState>()((set, get) => ({
  messages: [],
  isProcessing: false,
  error: null,

  sendMessage: async (content: string, includeScreen = true) => {
    const { addMessage } = get();

    // Add user message
    addMessage({ role: 'user', content });

    set({ isProcessing: true, error: null });

    // Show overlay and set thinking status
    try {
      await invoke('overlay_set_control', { controlling: true });
      await setOverlayStatus('thinking', 'Analyzing request...');
    } catch (e) {
      console.warn('Failed to show overlay:', e);
    }

    try {
      const response = await invoke<ActionResponse>('send_message', {
        message: content,
        includeScreen,
      });

      // Add assistant response
      addMessage({
        role: 'assistant',
        content: response.thought,
        action: response,
      });

      // Auto-execute if action is not 'none'
      if (response.action !== 'none') {
        await get().executeAction(response);
      }

      set({ isProcessing: false });

      // Hide overlay after completion
      await invoke('overlay_set_control', { controlling: false });
    } catch (e) {
      set({ error: String(e), isProcessing: false });
      addMessage({
        role: 'assistant',
        content: `Error: ${String(e)}`,
      });

      // Hide overlay on error
      try {
        await invoke('overlay_set_control', { controlling: false });
      } catch {}
    }
  },

  addMessage: (message) => {
    const newMessage: Message = {
      ...message,
      id: generateId(),
      timestamp: Date.now(),
    };
    set((state) => ({
      messages: [...state.messages, newMessage],
    }));
  },

  clearMessages: () => {
    set({ messages: [], error: null });
  },

  executeAction: async (action: ActionResponse) => {
    try {
      const x = action.params.x as number;
      const y = action.params.y as number;

      switch (action.action) {
        case 'mouse_move':
          await setOverlayStatus('moving', `Moving to (${x}, ${y})`);
          await updateOverlayCursor(x, y);
          await invoke('mouse_move', {
            x,
            y,
            smooth: action.params.smooth ?? true,
          });
          break;

        case 'mouse_click':
          await setOverlayStatus('clicking', `Clicking at (${x}, ${y})`);
          await updateOverlayCursor(x, y);
          await triggerClickEffect(x, y, action.params.button as string);
          await invoke('mouse_click', {
            x,
            y,
            button: action.params.button ?? 'left',
            double: action.params.clicks === 2,
          });
          break;

        case 'mouse_double_click':
          await setOverlayStatus('clicking', `Double-clicking at (${x}, ${y})`);
          await updateOverlayCursor(x, y);
          await triggerClickEffect(x, y, 'left');
          await invoke('mouse_click', {
            x,
            y,
            button: 'left',
            double: true,
          });
          break;

        case 'keyboard_type':
          await setOverlayStatus('typing', `Typing text...`);
          await invoke('keyboard_type', {
            text: action.params.text,
          });
          break;

        case 'keyboard_press':
          await setOverlayStatus('typing', `Pressing keys...`);
          await invoke('keyboard_press', {
            keys: action.params.keys,
          });
          break;

        case 'scroll':
          await setOverlayStatus('scrolling', `Scrolling ${action.params.direction}`);
          // Note: scroll implementation not yet complete in backend
          break;

        case 'get_screen_update':
          await setOverlayStatus('thinking', 'Updating screen...');
          break;

        case 'wait':
          await setOverlayStatus('thinking', 'Waiting...');
          await new Promise((resolve) =>
            setTimeout(resolve, (action.params.ms as number) || 1000)
          );
          break;

        case 'guide_search':
          await setOverlayStatus('thinking', 'Searching guides...');
          await invoke('guide_search', {
            query: action.params.query,
          });
          break;

        default:
          console.log('Unknown action:', action.action);
      }
    } catch (e) {
      console.error('Action execution error:', e);
      throw e;
    }
  },
}));

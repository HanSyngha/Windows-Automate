// Overlay store - manages visual effects state

import { create } from 'zustand';

export interface CursorPosition {
  x: number;
  y: number;
}

export interface ClickEffect {
  id: string;
  x: number;
  y: number;
  button: 'left' | 'right' | 'middle';
}

export type OverlayStatus = 'idle' | 'thinking' | 'moving' | 'clicking' | 'typing' | 'scrolling';

interface OverlayState {
  // Cursor state
  cursorPosition: CursorPosition;
  isAiControlling: boolean;
  showCursorGlow: boolean;

  // Click effects
  clickEffects: ClickEffect[];

  // Status
  status: OverlayStatus;
  statusMessage: string;

  // Actions
  setCursorPosition: (pos: CursorPosition) => void;
  setAiControlling: (controlling: boolean) => void;
  addClickEffect: (x: number, y: number, button?: 'left' | 'right' | 'middle') => void;
  removeClickEffect: (id: string) => void;
  setStatus: (status: OverlayStatus, message?: string) => void;
  reset: () => void;
}

const generateId = () => Math.random().toString(36).substring(2, 9);

export const useOverlayStore = create<OverlayState>()((set) => ({
  cursorPosition: { x: 0, y: 0 },
  isAiControlling: false,
  showCursorGlow: true,
  clickEffects: [],
  status: 'idle',
  statusMessage: '',

  setCursorPosition: (pos) => set({ cursorPosition: pos }),

  setAiControlling: (controlling) => set({ isAiControlling: controlling }),

  addClickEffect: (x, y, button = 'left') => {
    const id = generateId();
    set((state) => ({
      clickEffects: [...state.clickEffects, { id, x, y, button }],
    }));

    // Auto-remove after animation
    setTimeout(() => {
      set((state) => ({
        clickEffects: state.clickEffects.filter((e) => e.id !== id),
      }));
    }, 600);
  },

  removeClickEffect: (id) =>
    set((state) => ({
      clickEffects: state.clickEffects.filter((e) => e.id !== id),
    })),

  setStatus: (status, message = '') =>
    set({ status, statusMessage: message }),

  reset: () =>
    set({
      isAiControlling: false,
      clickEffects: [],
      status: 'idle',
      statusMessage: '',
    }),
}));

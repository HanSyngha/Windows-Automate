// Overlay App - main component for the overlay window

import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { CursorGlow } from './CursorGlow';
import { ClickRipple } from './ClickRipple';
import { StatusIndicator } from './StatusIndicator';
import { useOverlayStore } from '../../stores/overlayStore';

interface OverlayEvent {
  type: 'cursor_move' | 'click' | 'status' | 'control';
  payload: {
    x?: number;
    y?: number;
    button?: 'left' | 'right' | 'middle';
    status?: string;
    message?: string;
    controlling?: boolean;
  };
}

export function OverlayApp() {
  const {
    setCursorPosition,
    setAiControlling,
    addClickEffect,
    setStatus,
  } = useOverlayStore();

  useEffect(() => {
    // Listen for overlay events from the main process
    const unlisten = listen<OverlayEvent>('overlay-event', (event) => {
      const { type, payload } = event.payload;

      switch (type) {
        case 'cursor_move':
          if (payload.x !== undefined && payload.y !== undefined) {
            setCursorPosition({ x: payload.x, y: payload.y });
          }
          break;

        case 'click':
          if (payload.x !== undefined && payload.y !== undefined) {
            addClickEffect(payload.x, payload.y, payload.button || 'left');
          }
          break;

        case 'status':
          if (payload.status) {
            setStatus(payload.status as any, payload.message);
          }
          break;

        case 'control':
          if (payload.controlling !== undefined) {
            setAiControlling(payload.controlling);
          }
          break;
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [setCursorPosition, setAiControlling, addClickEffect, setStatus]);

  return (
    <div className="w-screen h-screen overflow-hidden" style={{ background: 'transparent' }}>
      <CursorGlow />
      <ClickRipple />
      <StatusIndicator />
    </div>
  );
}

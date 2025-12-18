// Cursor glow effect component

import { motion } from 'motion/react';
import { useOverlayStore } from '../../stores/overlayStore';
import { useShallow } from 'zustand/shallow';

export function CursorGlow() {
  const { cursorPosition, isAiControlling } = useOverlayStore(
    useShallow((state) => ({
      cursorPosition: state.cursorPosition,
      isAiControlling: state.isAiControlling,
    }))
  );

  if (!isAiControlling) return null;

  return (
    <motion.div
      className="fixed pointer-events-none"
      style={{
        left: cursorPosition.x,
        top: cursorPosition.y,
        transform: 'translate(-50%, -50%)',
      }}
      initial={{ scale: 0, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      exit={{ scale: 0, opacity: 0 }}
    >
      {/* Outer glow */}
      <motion.div
        className="absolute rounded-full"
        style={{
          width: 60,
          height: 60,
          left: -30,
          top: -30,
          background: 'radial-gradient(circle, rgba(99, 102, 241, 0.3) 0%, transparent 70%)',
        }}
        animate={{
          scale: [1, 1.2, 1],
          opacity: [0.5, 0.8, 0.5],
        }}
        transition={{
          duration: 1.5,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {/* Middle ring */}
      <motion.div
        className="absolute rounded-full border-2 border-indigo-400/50"
        style={{
          width: 30,
          height: 30,
          left: -15,
          top: -15,
        }}
        animate={{
          scale: [1, 1.3, 1],
          opacity: [0.6, 0.3, 0.6],
        }}
        transition={{
          duration: 1,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {/* Inner dot */}
      <motion.div
        className="absolute rounded-full bg-indigo-400"
        style={{
          width: 8,
          height: 8,
          left: -4,
          top: -4,
          boxShadow: '0 0 10px 3px rgba(99, 102, 241, 0.6)',
        }}
        animate={{
          scale: [1, 1.2, 1],
        }}
        transition={{
          duration: 0.5,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />
    </motion.div>
  );
}
